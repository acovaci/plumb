use clap::{Parser, Subcommand};

use plumb::{core::config::PlumbConfig, error::Error};

mod cli;

#[derive(Parser, Debug)]
#[command(
    name = "plumb",
    about = "A project manager for your development projects."
)]

struct Args {
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
        command: ProjectCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ProjectCommand {
    /// List all projects.
    List,
}

pub fn main() {
    let cli = Args::parse();

    match run(cli) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(cli: Args) -> Result<(), Error> {
    let config = PlumbConfig::load(None).unwrap_or_else(|e| {
        eprintln!("Error loading configuration: {}", e);
        std::process::exit(1);
    });

    match cli.command {
        Some(Command::Version) => {
            cli::version::version()?;
        }
        Some(Command::Project { command }) => match command {
            ProjectCommand::List => {
                cli::project::list(config)?;
            }
        },
        _ => Err(Error::InvalidArguments)?,
    }

    Ok(())
}
