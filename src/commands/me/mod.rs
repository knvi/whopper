mod list;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    List(list::Options)
}

#[derive(Debug, Parser)]
#[clap(about = "Interact with user data")]
pub struct Options {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run(options: Options) -> Result<()> {
    match options.commands {
        Commands::List(options) => list::run(options).await,
    }
}