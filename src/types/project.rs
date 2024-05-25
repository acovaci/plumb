use std::path::PathBuf;

use super::Location;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Project {
    name: String,
    location: Location,
}

impl Project {
    pub fn new(name: &str, location: Location) -> Self {
        Self {
            name: name.to_string(),
            location,
        }
    }

    pub fn new_in_location(name: &str, location: &Location) -> Self {
        Self {
            name: name.to_string(),
            location: location.join(name),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn path(&self) -> &PathBuf {
        self.location.path()
    }
}
