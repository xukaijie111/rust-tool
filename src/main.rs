mod link;
mod list;
mod package;
mod util;
mod clean;

use anyhow::Result;
use link::LinkCommand;
use list::ListCommand;
use clean::CleanCommand;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
enum SubCommands {
    Link(LinkCommand),
    List(ListCommand),
    Clear(CleanCommand),
}

#[derive(Parser, Clone, Debug)]
pub struct Command {
    #[command(subcommand)]
    command: SubCommands,
}

fn main() -> Result<()> {
    let command: Command = Command::parse();

    match command.command {
        SubCommands::Link(ref options) => LinkCommand::run(&command, options),
        SubCommands::List(ref options) => ListCommand::run(&command, options),
        SubCommands::Clear(ref options) => CleanCommand::run(&command, options),
        #[allow(unreachable_patterns)]
        _ => Ok(()),
    }
}
