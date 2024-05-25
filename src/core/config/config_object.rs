use std::collections::HashMap;

use crate::{
    error::{PlumbError, Res},
    types::{location::LocationKey, Location, Project},
};

use super::project::ProjectBuilder;

#[derive(Debug, Clone, PartialEq)]
pub struct PlumbConfig {
    pub(super) locations: PlumbLocations,
    pub(super) projects: Vec<Project>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct PlumbLocations {
    pub defaults: PlumbDefaultLocations,
    pub projects: HashMap<LocationKey, Location>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct PlumbDefaultLocations {
    pub projects: Option<Location>,
}

impl PlumbConfig {
    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn new_project(&mut self, name: &str, location: Option<Location>) -> Res<Project> {
        let builder = self.project_builder()?;
        let project = builder.build(name, location);
        self.add_project(project.clone());
        Ok(project)
    }

    pub fn project_builder(&self) -> Res<ProjectBuilder> {
        let default_location = self
            .default_project_location()
            .ok_or_else(|| PlumbError::NoDefaultLocation("projects".into()))?
            .clone();

        Ok(ProjectBuilder::new(default_location))
    }

    pub fn default_project_location(&self) -> Option<&Location> {
        self.locations.defaults.projects.as_ref()
    }

    pub fn list_project_locations(&self) -> Vec<&Location> {
        self.locations.projects.values().collect()
    }

    pub fn get_location(&self, key: LocationKey) -> Option<&Location> {
        self.locations.projects.get(&key)
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }
}
