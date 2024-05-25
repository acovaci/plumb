use std::path::PathBuf;

pub type LocationKey = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Location {
    key: LocationKey,
    path: PathBuf,
    name: Option<String>,
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

    pub fn join(&self, path: &str) -> Location {
        Location::new(self.path.join(path), None)
    }
}
