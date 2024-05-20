use std::path::PathBuf;

use crate::error::Error;

pub fn get_config_file() -> Result<PathBuf, Error> {
    if let Some(override_path) = get_config_file_override() {
        return Ok(override_path);
    }

    let config_dir = get_config_dir()?;
    let config_file = get_config_filename()?;
    Ok(config_dir.join(config_file))
}

pub fn get_config_filename() -> Result<String, Error> {
    const DEFAULT_CONFIG: &str = "plumb.yaml";
    Ok(DEFAULT_CONFIG.to_string())
}

pub fn get_config_dir() -> Result<PathBuf, Error> {
    if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(config_dir));
    }

    let home_dir = get_home_dir()?;
    Ok(home_dir.join(".config"))
}

pub fn get_config_file_override() -> Option<PathBuf> {
    if let Ok(config_env) = std::env::var("PLUMB_CONFIG") {
        return Some(config_env.into());
    }

    if cfg!(test) || cfg!(debug_assertions) {
        let cwd = std::env::current_dir().unwrap();
        return Some(cwd.join("tests").join(".config").join("plumb.yaml"));
    }

    None
}

pub fn get_home_dir() -> Result<PathBuf, Error> {
    if cfg!(test) || cfg!(debug_assertions) {
        let cwd = std::env::current_dir()?;
        return Ok(cwd.join("tests"));
    }

    home::home_dir().ok_or(Error::HomeDirNotFound)
}
