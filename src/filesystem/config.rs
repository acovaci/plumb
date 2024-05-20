use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    core::{config::PlumbSettings, project::ProjectBuilder, settings::SettingsBuilder},
    error::Error,
    types::{location::DefaultLocation, Location, Project},
};

use super::user::get_config_file;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigFile {
    pub locations: LocationsConfigChunk,
    pub projects: Vec<ProjectChunk>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocationsConfigChunk {
    pub(self) projects: Vec<LocationChunk>,
    pub(self) defaults: LocationDefaultsConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct LocationChunk {
    path: PathBuf,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct LocationDefaultsConfig {
    projects: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectChunk {
    pub(self) name: String,
    pub(self) location: Option<PathBuf>,
}

impl ConfigFile {
    pub fn load(path: Option<&Path>) -> Result<ConfigFile, Error> {
        let path = path.map(PathBuf::from).unwrap_or(get_config_file()?);

        let config = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?
            .try_deserialize::<ConfigFile>()?;

        Ok(config)
    }

    pub fn parse_locations(&self) -> Vec<Location> {
        self.locations
            .projects
            .iter()
            .map(|l| Location::new(l.path.clone(), l.name.clone()))
            .collect()
    }

    pub fn parse_projects(&self, builder: ProjectBuilder) -> Vec<Project> {
        self.projects
            .iter()
            .map(|p| {
                builder.build(
                    p.name.as_str(),
                    p.location.as_ref().map(|l| Location::new(l.clone(), None)),
                )
            })
            .collect()
    }

    pub fn parse_settings(&self, builder: &mut SettingsBuilder) -> Result<PlumbSettings, Error> {
        let default_locations = self.locations.defaults.clone();
        builder.add_default_location(DefaultLocation::Projects, default_locations.projects);
        builder.build()
    }
}
