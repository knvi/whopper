#![warn(clippy::pedantic)]
#![allow(dead_code)]

pub(crate) mod commands;
pub(crate) mod util;
pub(crate) mod config;
pub(crate) mod state;
pub(crate) mod vars;
pub(crate) mod api;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use commands::{Commands, handle_command};

#[derive(Debug, Parser)]
#[clap(
    name = "whopper",
    about = "Interact with whop using the CLI ☁️",
    version = crate::vars::CLI_VERSION,
    author = "kunevi"
)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run() -> Result<()> {
    dotenv().ok(); // load env variables from .env

    let cli = CLI::parse();

    if let Err(err) = handle_command(cli.commands).await {
        eprintln!("Error: {err}");
    }

    Ok(())
}