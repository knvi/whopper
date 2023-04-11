pub mod auth;
pub mod whoami;
pub mod product;
pub mod biz;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Auth(auth::Options),
    Whoami(whoami::Options),
    Product(product::Options),
    Biz(biz::Options),
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Auth(options) => auth::run(options).await,
        Commands::Whoami(options) => whoami::run(options).await,
        Commands::Product(options) => product::run(options).await,
        Commands::Biz(options) => biz::run(options).await,
    }
}