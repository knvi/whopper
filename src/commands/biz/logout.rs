use anyhow::Result;
use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser)]
#[clap(about = "Logout of whop with your business")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {
    let mut config = Config::load();
    config.delete_biz_id();
    config.save()?;

    println!("🚀 Successfully logged out!");

    Ok(())
}