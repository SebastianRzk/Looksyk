use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq)]
pub struct ApplicationVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApplicationVersion {
    pub fn new(version: &str) -> Self {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            panic!("Invalid version format");
        }
        ApplicationVersion {
            major: parts[0].parse().unwrap(),
            minor: parts[1].parse().unwrap(),
            patch: parts[2].parse().unwrap(),
        }
    }
}

impl Ord for ApplicationVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.major != other.major {
            return self.major.cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.cmp(&other.minor);
        }
        self.patch.cmp(&other.patch)
    }
}

impl PartialOrd for ApplicationVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for ApplicationVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_version() {
        let version = ApplicationVersion::new("1.2.3");
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_application_version_comparison() {
        let version1 = ApplicationVersion::new("1.2.3");
        let version2 = ApplicationVersion::new("1.2.4");
        assert!(version1 < version2);

        let version3 = ApplicationVersion::new("1.3.0");
        assert!(version1 < version3);

        let version4 = ApplicationVersion::new("2.0.0");
        assert!(version1 < version4);
    }

    #[test]
    fn test_application_version_display() {
        let version = ApplicationVersion::new("1.2.3");
        assert_eq!(version.to_string(), "1.2.3");
    }
}
