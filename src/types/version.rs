use std::fmt::{Display, Formatter};

use crate::error::{PlumbError, Res};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl SemanticVersion {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn from_env() -> Res<Self> {
        Self::try_from(env!("CARGO_PKG_VERSION"))
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

impl TryFrom<&str> for SemanticVersion {
    type Error = PlumbError;

    fn try_from(version: &str) -> Res<Self> {
        let parts: Vec<&str> = version.split('.').collect();
        let (major, minor, patch) = match (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
            (Ok(major), Ok(minor), Ok(patch)) => (major, minor, patch),
            _ => return Err(PlumbError::VersionParseError(version.to_string())),
        };

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let version = SemanticVersion::try_from("0.1.0").unwrap();
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
