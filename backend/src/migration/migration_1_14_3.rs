pub fn migriere_1_14_3() {
    let old_path = crate::io::fs::basic_folder::home_directory()
        .join(".local")
        .join("share")
        .join("looksyk")
        .join("config.json");
    let new_path = crate::io::fs::basic_folder::config_directory().join("config.json");
    if std::path::Path::new(&old_path).exists() && !std::path::Path::new(&new_path).exists() {
        std::fs::create_dir_all(crate::io::fs::basic_folder::config_directory()).unwrap();
        std::fs::copy(&old_path, &new_path).unwrap();
    }
}
