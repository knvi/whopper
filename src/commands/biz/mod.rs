mod init;
mod logout;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(init::Options),
    Logout(logout::Options),
}

#[derive(Debug, Parser)]
#[clap(about = "Interact with your business")]
pub struct Options {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run(options: Options) -> Result<()> {
    match options.commands {
        Commands::Init(options) => init::run(options).await,
        Commands::Logout(options) => logout::run(options).await,
    }
}