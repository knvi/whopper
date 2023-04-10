pub mod login;
pub mod logout;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(alias = "li")]
    Login(login::Options),
    #[clap(alias = "lo")]
    Logout(logout::Options),
}

#[derive(Debug, Parser)]
#[clap(about = "Authenticate to whop.")]
pub struct Options {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run(options: Options) -> Result<()> {
    match options.commands {
        Commands::Login(options) => login::run(options).await,
        Commands::Logout(options) => logout::run(options).await,
    }
}