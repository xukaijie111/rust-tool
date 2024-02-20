
mod link;
mod package;
mod util;

use link :: LinkCommand;
use anyhow::Result;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
enum SubCommands {
    Link(LinkCommand)
}

#[derive(Parser, Clone, Debug)]
pub struct Command {
    #[command(subcommand)]
    command: SubCommands,

}



fn main()->Result<()> {
    println!("Hello, world!");
    
    let command: Command = Command::parse();

    match command.command {
        SubCommands::Link(ref options) => LinkCommand::run(&command, options),
      
        #[allow(unreachable_patterns)]
        _ => Ok(()),
    }
}
