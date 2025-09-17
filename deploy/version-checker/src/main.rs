use std::fs;

fn extract_version_from_json(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    json.get("version")?.as_str().map(|s| s.to_string())
}


fn extract_version_from_metainfo_xml(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let doc = roxmltree::Document::parse(&content).ok()?;
    let releases = doc.descendants().find(|n| n.has_tag_name("releases"))?;
    let release = releases
        .children()
        .find(|n| n.has_tag_name("release") && n.attribute("version").is_some())?;
    release.attribute("version").map(|s| s.to_string())
}
fn extract_version_from_cargo(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if line.trim_start().starts_with("version") {
            return line
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn extract_version_from_cargo_lock(path: &str, package_name: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let mut in_package = false;
    let mut found_name = false;
    for line in content.lines() {
        if line.trim() == "[[package]]" {
            in_package = true;
            found_name = false;
            continue;
        }
        if in_package && line.trim_start().starts_with("name") && line.split('=').nth(1)?.trim().trim_matches('"') == package_name {
            found_name = true;
        }
        if in_package && found_name && line.trim_start().starts_with("version") {
            return line
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
        if in_package && line.trim().is_empty() {
            in_package = false;
        }
    }
    None
}

fn extract_version_from_pkgbuild(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if line.trim_start().starts_with("pkgver=") {
            return line.split('=').nth(1).map(|s| s.trim().to_string());
        }
    }
    None
}

fn extract_version_from_yaml_source_url(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if line.contains("github.com") && line.contains("/releases/download/") {
            let parts: Vec<&str> = line.split('/').collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "download" && i + 1 < parts.len() {
                    return Some(parts[i + 1].trim_matches('\'').chars().skip(1).collect());
                }
            }
        }
    }
    None
}

fn is_version_in_changelog(path: &str, version: &str) -> bool {
    let content = fs::read_to_string(path).unwrap_or_default();
    content.lines().any(|line| line.contains(version))
}

fn is_version_published(version: &str) -> Option<bool> {
    use std::process::Command;
    let output = Command::new("git")
        .args(["tag", "--list"])
        .output()
        .ok()?;
    let tags = String::from_utf8_lossy(&output.stdout);
    Some(tags.lines().any(|tag| tag.trim() == version))
}

fn main() {
    let paths = [
        (
            "application-wrapper/package.json",
            extract_version_from_json("application-wrapper/Looksyk/package.json"),
        ),
        (
            "application-wrapper/package-lock.json",
            extract_version_from_json("application-wrapper/Looksyk/package-lock.json"),
        ),
        (
            "frontend/package.json",
            extract_version_from_json("frontend/looksyk/package.json"),
        ),
        (
            "frontend/package-lock.json",
            extract_version_from_json("frontend/looksyk/package-lock.json"),
        ),
        (
            "backend/Cargo.toml",
            extract_version_from_cargo("backend/Cargo.toml"),
        ),
        (
            "backend/Cargo.lock",
            extract_version_from_cargo_lock("backend/Cargo.lock", "looksyk"),
        ),
        ("PKGBUILD", extract_version_from_pkgbuild("PKGBUILD")),
//        (
//            "looksyk.yml",
//            extract_version_from_yaml_source_url("de.sebastianruziczka.looksyk.yml"),
//        ),
        (
            "de.sebastianruziczka.looksyk.metainfo.xml",
            extract_version_from_metainfo_xml("de.sebastianruziczka.looksyk.metainfo.xml"),
        ),
    ];

    let versions: Vec<_> = paths.iter().filter_map(|(_, v)| v.as_ref()).collect();

    if versions.windows(2).all(|w| w[0] == w[1]) {
        println!("All versions match {}", versions[0]);
    } else {
        for (name, version) in &paths {
            println!(
                "{:<45}: {}",
                name,
                version.as_deref().unwrap_or("Not found")
            );
        }
        eprintln!("Error: Not all versions match!");
        std::process::exit(1);
    }
    let formatted_version = format!("v{}", versions[0]);
    let version_in_git = is_version_published(formatted_version.as_str());

    if version_in_git.is_none() {
        eprintln!("Error: Failed to check if version is published in git.");
        std::process::exit(1);
    }

    if version_in_git.unwrap() {
        eprintln!("Error: version {} already published!", versions[0]);
        std::process::exit(1);
    }

    println!("Version {} is not published yet.", versions[0]);

    let version_in_changelog = is_version_in_changelog("docs/changelog.md", formatted_version.as_str());
    if !version_in_changelog {
        eprintln!("Error: Version {} not found in changelog!", formatted_version);
        std::process::exit(1);
    }
    println!("Version {} found in changelog.", formatted_version);

    println!("All checks passed successfully.");
}
