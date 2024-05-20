use std::path::PathBuf;

pub type LocationKey = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Location {
    key: LocationKey,
    path: PathBuf,
    name: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DefaultLocation {
    Projects,
}

impl Location {
    pub fn new(path: PathBuf, name: Option<String>) -> Self {
        Self {
            path,
            name: name.clone(),
            key: name.unwrap_or("".to_string()),
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn key(&self) -> &LocationKey {
        &self.key
    }
}
