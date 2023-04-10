use anyhow::Result;
use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser)]
#[clap(about = "Logout of whop")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {
    let mut config = Config::load();
    config.delete_user();
    config.save()?;

    println!("🚀 Successfully logged out of whop!");

    Ok(())
}