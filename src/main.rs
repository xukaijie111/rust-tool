mod link;
mod package;
mod util;

use anyhow::Result;
use link::LinkCommand;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
enum SubCommands {
    Link(LinkCommand),
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

        #[allow(unreachable_patterns)]
        _ => Ok(()),
    }
}
