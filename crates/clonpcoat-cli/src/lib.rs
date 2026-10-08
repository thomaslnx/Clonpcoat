mod dev;
mod fmt;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clonpcoat")]
pub struct ClonpcoatCli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Format clonpcoat `view!` macros
    Fmt(fmt::DevCommand),

    /// Start a development server
    Dev(dev::DevCommand),
}

pub async fn run() {
    let cli = ClonpcoatCli::parse();

    match cli.command {
        Command::Fmt(cmd) => cmd.run().await,
        Command::Dev(cmd) => cmd.run().await,
    }
}
