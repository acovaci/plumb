use std::collections::HashMap;

use crate::{
    error::{PlumbError, Res},
    fs::config::{
        ConfigFile, LocationChunk, LocationDefaultsChunk, LocationsConfigChunk, ProjectChunk,
    },
    types::{Location, Project},
};

use super::{
    config_object::{PlumbDefaultLocations, PlumbLocations},
    PlumbConfig,
};

pub trait ConfigTranscriber {
    fn load(config_file: ConfigFile) -> Res<Self>
    where
        Self: Sized;
    fn dump(&self) -> ConfigFile;
}

impl ConfigTranscriber for PlumbConfig {
    fn dump(&self) -> ConfigFile {
        let projects = self
            .projects
            .iter()
            .map(|project| {
                let name = project.name().to_string();
                let path = project.location().path().to_path_buf();
                ProjectChunk {
                    name,
                    path: Some(path),
                }
            })
            .collect();

        let default_project_location_key = self
            .default_project_location()
            .map(|location| location.key().clone());

        let location_defaults =
            default_project_location_key.map(|projects| LocationDefaultsChunk { projects });

        let project_locations = self
            .locations
            .projects
            .values()
            .map(|location| {
                let name = location.name().clone();
                let path = location.path().to_path_buf();
                LocationChunk { name, path }
            })
            .collect();

        let locations = LocationsConfigChunk {
            projects: project_locations,
            defaults: location_defaults,
        };

        ConfigFile {
            locations,
            projects,
        }
    }

    fn load(config_file: ConfigFile) -> Res<Self> {
        let locations_config_chunk = config_file.locations;
        let project_locations: HashMap<String, Location> = locations_config_chunk
            .projects
            .iter()
            .map(|location| {
                (
                    location
                        .name
                        .clone()
                        .unwrap_or(location.path.to_string_lossy().into_owned()),
                    Location::new(location.path.clone(), location.name.clone()),
                )
            })
            .collect();
        let default_project_location_key = locations_config_chunk
            .defaults
            .ok_or(PlumbError::NoDefaultLocation(String::new()))?
            .projects;
        let default_project_location = project_locations
            .get(&default_project_location_key)
            .cloned()
            .ok_or(PlumbError::NoDefaultLocation("projects".into()))?;
        let locations = PlumbLocations {
            defaults: PlumbDefaultLocations {
                projects: Some(default_project_location.clone()),
            },
            projects: project_locations,
        };

        let projects = config_file
            .projects
            .into_iter()
            .map(|project| {
                Project::new(
                    &project.name,
                    Location::new(
                        project
                            .path
                            .unwrap_or(default_project_location.join(&project.name).path().clone()),
                        None,
                    ),
                )
            })
            .collect();

        Ok(Self {
            locations,
            projects,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{core::config::manager::ConfigManager, testing::ExpectedFile};

    #[test]
    fn test_transcribe() {
        let target_file = ExpectedFile::new("tests/fixtures/transcriber_dump.yml".into());
        let temp_file = target_file.temp_file();

        let source_file = PathBuf::from("tests/fixtures/transcriber_load.yml");
        let mut manager = ConfigManager::new(Some(&source_file)).unwrap();
        let source_config = manager.load().unwrap();

        manager.update(source_config.clone()).unwrap();
        manager.dump_to(&temp_file).unwrap();

        let target_config = ConfigManager::new(Some(&temp_file))
            .unwrap()
            .load()
            .unwrap();

        assert_eq!(source_config, target_config);

        // todo: Either implement assert in a way that ignores key/vec order or force order on the
        // todo: expected file
        //target_file.assert();
    }
}
