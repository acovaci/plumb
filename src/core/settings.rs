use std::collections::HashMap;

use crate::error::Error;
use crate::types::DefaultLocation;

use super::config::{PlumbConfig, PlumbSettings};

pub struct SettingsBuilder<'a> {
    config: &'a PlumbConfig,
    default_locations: HashMap<DefaultLocation, String>,
}

impl SettingsBuilder<'_> {
    pub fn new(config: &PlumbConfig) -> SettingsBuilder {
        SettingsBuilder {
            config,
            default_locations: HashMap::new(),
        }
    }

    pub fn add_default_location(&mut self, kind: DefaultLocation, location: String) {
        self.default_locations.insert(kind, location);
    }

    pub fn build(&self) -> Result<PlumbSettings, Error> {
        let default_locations = self
            .default_locations
            .iter()
            .filter_map(|(k, v)| {
                let location = self.config.get_location(v.to_string());
                location.map(|l| (k.clone(), l.clone()))
            })
            .collect();

        Ok(PlumbSettings::new(default_locations))
    }
}
