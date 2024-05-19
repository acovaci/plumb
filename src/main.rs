use clap::{Parser, Subcommand};
use plumb::command_version;
use plumb::config::load_config;
use plumb::project;

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

    let config = load_config().unwrap();

    match cli.command {
        Some(Command::Version) => {
            command_version();
        }
        Some(Command::Project { command }) => match command {
            ProjectCommand::List => {
                project::list(config);
            }
        },
        _ => {
            println!("Invalid arguments. Try `plumb help` for more information.");
        }
    }
}
