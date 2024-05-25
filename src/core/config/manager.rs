use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    error::{PlumbError, Res},
    fs::{config::ConfigFile, user::get_config_file},
};

use super::{transcriber::ConfigTranscriber, PlumbConfig};

// todo: clean up this structure somehow
#[derive(Debug, Clone)]
pub struct ConfigManager {
    path: PathBuf,
    builder: config::ConfigBuilder<config::builder::DefaultState>,
    config: Option<PlumbConfig>,
}

impl ConfigManager {
    pub fn new(path: Option<&Path>) -> Res<Self> {
        let default_path = get_config_file()?;
        let path = path.unwrap_or(&default_path);
        let file = config::File::from(path);
        let builder = config::Config::builder().add_source(file);

        Ok(Self {
            path: path.into(),
            builder,
            config: None,
        })
    }

    pub fn try_load(path: Option<&Path>) -> Res<Self> {
        let mut manager = Self::new(path)?;
        manager.load()?;
        Ok(manager)
    }

    pub fn config(&self) -> Res<&PlumbConfig> {
        self.config.as_ref().ok_or(PlumbError::EmptyConfigFile)
    }

    pub fn update(&mut self, config: PlumbConfig) -> Res<()> {
        self.config = Some(config);
        self.dump()
    }

    pub fn load(&mut self) -> Res<PlumbConfig> {
        let config: ConfigFile = self.builder.build_cloned()?.try_deserialize()?;
        let plumb_config = PlumbConfig::load(config)?;
        self.config = Some(plumb_config.clone());
        Ok(plumb_config)
    }

    pub fn dump_to(&self, path: &Path) -> Res<()> {
        let config = self.config.as_ref().ok_or(PlumbError::EmptyConfigFile)?;
        let content = serde_yaml::to_string(&config.dump())?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn dump(&self) -> Res<()> {
        self.dump_to(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::testing::ExpectedFile;

    use super::*;

    #[test]
    fn test_load_then_dump() {
        let target_file = ExpectedFile::new("tests/fixtures/manager_dump.yml".into());
        let temp_file = target_file.temp_file();

        let source_file = PathBuf::from("tests/fixtures/manager_load.yml");
        let mut manager = ConfigManager::new(Some(&source_file)).unwrap();

        let source_config = manager.load().unwrap();

        manager.dump_to(&temp_file).unwrap();

        let target_config = ConfigManager::new(Some(&temp_file))
            .unwrap()
            .load()
            .unwrap();

        assert_eq!(source_config, target_config);

        // todo: Figure out a way to assert this ignoring the vec/key order
        // target_file.assert();
    }
}
