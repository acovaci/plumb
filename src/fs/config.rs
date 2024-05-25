use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct ConfigFile {
    pub locations: LocationsConfigChunk,
    pub projects: Vec<ProjectChunk>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct LocationsConfigChunk {
    pub projects: Vec<LocationChunk>,
    pub defaults: Option<LocationDefaultsChunk>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct LocationChunk {
    pub path: PathBuf,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct LocationDefaultsChunk {
    pub projects: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct ProjectChunk {
    pub name: String,
    pub path: Option<PathBuf>,
}
