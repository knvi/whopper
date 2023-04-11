pub mod auth;
pub mod whoami;
pub mod product;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Auth(auth::Options),
    Whoami(whoami::Options),
    Product(product::Options),
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Auth(options) => auth::run(options).await,
        Commands::Whoami(options) => whoami::run(options).await,
        Commands::Product(options) => product::run(options).await,
    }
}