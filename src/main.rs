use clap::{Parser, Subcommand};
use plumb::command_version;

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
}

pub fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Command::Version) => {
            command_version();
        }
        _ => {
            println!("Invalid arguments. Try `plumb help` for more information.");
        }
    }
}
