use thiserror::Error;

pub type Res<T> = Result<T, PlumbError>;

#[derive(Debug, Error)]
pub enum PlumbError {
    #[error("Could not find home directory")]
    HomeDirNotFound,

    #[error("Invalid config file format: {0}")]
    InvalidConfigFileFormat(String),

    #[error("Attempting to write an empty configuration file")]
    EmptyConfigFile,

    #[error("Attempting to write with no configuration manager")]
    NoConfigManager,

    #[error("Could not parse version string: {0}")]
    VersionParseError(String),

    #[error("Could not load configuration file: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid arguments. Please see `plumb help` for more information.")]
    InvalidArguments,

    #[error("No default location found for {0}")]
    NoDefaultLocation(String),

    #[error("Could not execute git command: {0}")]
    GitError(#[from] git2::Error),

    #[error("Could not write the configuration file: {0}")]
    WriteConfigError(#[from] serde_yaml::Error),

    #[error("Unknown error: {0}")]
    UnknownError(#[from] std::boxed::Box<dyn std::error::Error>),
}
