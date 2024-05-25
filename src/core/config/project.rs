use crate::types::{Location, Project};

pub struct ProjectBuilder {
    location: Location,
}

impl ProjectBuilder {
    pub fn new(default_location: Location) -> Self {
        Self {
            location: default_location,
        }
    }

    pub fn build(&self, name: &str, location: Option<Location>) -> Project {
        Project::new(
            name,
            location.unwrap_or({
                let path = self.location.path().join(name);
                Location::new(path, None)
            }),
        )
    }
}
