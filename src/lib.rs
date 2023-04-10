#![warn(clippy::pedantic)]

pub(crate) mod commands;
pub(crate) mod util;
pub(crate) mod config;
pub(crate) mod state;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "whopper",
    about = "Interact with whop using the CLI ☁️",
    version = crate::config::CLI_VERSION,
    author = "kunevi"
)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run() -> Result<()> {
    let cli = CLI::parse();

    todo!()

    Ok(())
}