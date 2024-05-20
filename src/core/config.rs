use std::{collections::HashMap, path::Path};

use crate::{
    error::Error,
    filesystem::config::ConfigFile,
    types::{location::LocationKey, DefaultLocation, Location, Project},
};

use super::{project::ProjectBuilder, settings::SettingsBuilder};

pub struct PlumbConfig {
    locations: HashMap<LocationKey, Location>,
    settings: PlumbSettings,
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Default)]
pub struct PlumbSettings {
    pub(self) default_project_location: Option<Location>,
}

impl PlumbSettings {
    pub fn new(default_locations: HashMap<DefaultLocation, Location>) -> Self {
        Self {
            default_project_location: default_locations.get(&DefaultLocation::Projects).cloned(),
        }
    }
}

impl PlumbConfig {
    pub fn new(locations: Vec<Location>) -> Self {
        Self {
            locations: locations
                .into_iter()
                .map(|l| (l.key().clone(), l))
                .collect(),
            projects: vec![],
            settings: PlumbSettings::default(),
        }
    }

    pub fn load(path: Option<&Path>) -> Result<Self, Error> {
        let config = ConfigFile::load(path)?;
        let locations = config.parse_locations();
        let mut plumb_config = Self::new(locations);

        let mut settings_builder = plumb_config.settings_builder();

        let settings = config.parse_settings(&mut settings_builder)?;

        plumb_config.settings = settings;
        let project_builder = plumb_config
            .project_builder()
            .unwrap_or_else(|| panic!("No project builder found"));

        let projects = config.parse_projects(project_builder);

        for project in projects {
            plumb_config.add_project(project);
        }

        Ok(plumb_config)
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn project_builder(&self) -> Option<ProjectBuilder> {
        Some(ProjectBuilder::new(
            self.default_location_for(DefaultLocation::Projects)?
                .clone(),
        ))
    }

    fn settings_builder(&self) -> SettingsBuilder {
        SettingsBuilder::new(self)
    }

    pub fn list_locations(&self) -> Vec<&Location> {
        self.locations.values().collect()
    }

    pub fn get_location(&self, key: LocationKey) -> Option<&Location> {
        self.locations.get(&key)
    }

    pub fn default_location_for(&self, kind: DefaultLocation) -> Option<&Location> {
        match kind {
            DefaultLocation::Projects => self.settings.default_project_location.as_ref(),
        }
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }
}
