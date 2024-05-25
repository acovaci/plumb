use clap::{Parser, Subcommand};
use plumb::{
    core::config::manager::ConfigManager,
    error::{PlumbError, Res},
};

pub mod project;
pub mod version;

#[derive(Parser, Debug)]
#[command(
    name = "plumb",
    about = "A project manager for your development projects."
)]

pub struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Prints the version of the CLI.
    Version,

    /// Interact with project configurations.
    Project {
        #[command(subcommand)]
        command: project::ProjectCommand,
    },
}

pub fn run() -> Res<()> {
    let cli = Args::parse();

    let config = ConfigManager::try_load(None)?;

    match cli.command {
        Some(Command::Version) => version::version(),
        Some(Command::Project { command }) => project::run(config, command),
        _ => Err(PlumbError::InvalidArguments)?,
    }
}
