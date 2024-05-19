use thiserror::Error;

pub type Error = PlumbError;

#[derive(Debug, Error)]
pub enum PlumbError {
    #[error("Could not find home directory.")]
    HomeDirNotFound,

    #[error("Could not load configuration file.")]
    ConfigError(#[from] config::ConfigError),

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
