use thiserror::Error;

pub type Error = PlumbError;

#[derive(Debug, Error)]
pub enum PlumbError {
    #[error("Could not find home directory")]
    HomeDirNotFound,

    #[error("Could not parse version string: {0}")]
    VersionParseError(String),

    #[error("Could not load configuration file: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid arguments. Please see `plumb help` for more information.")]
    InvalidArguments,
}
