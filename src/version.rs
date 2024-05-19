use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

pub fn version() -> SemanticVersion {
    SemanticVersion::from_env()
}

impl SemanticVersion {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    fn from_env() -> Self {
        Self::from(env!("CARGO_PKG_VERSION"))
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }

    pub fn patch(&self) -> u8 {
        self.patch
    }
}

impl From<&str> for SemanticVersion {
    fn from(version: &str) -> Self {
        let parts: Vec<&str> = version.split('.').collect();
        let major = parts[0].parse().unwrap();
        let minor = parts[1].parse().unwrap();
        let patch = parts[2].parse().unwrap();

        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version().to_string(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_from_str() {
        let version = SemanticVersion::from("0.1.0");
        assert_eq!(version.major(), 0);
        assert_eq!(version.minor(), 1);
        assert_eq!(version.patch(), 0);
    }

    #[test]
    fn test_display() {
        let version = SemanticVersion::new(0, 1, 0);
        assert_eq!(version.to_string(), "0.1.0");
    }
}
