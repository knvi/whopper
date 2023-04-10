pub mod auth;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Auth(auth::Options)
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Auth(options) => auth::run(options).await,
    }
}