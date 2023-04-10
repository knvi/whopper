use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Logout of whop")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {
    todo!("Auth not implemented.");

    Ok(())
}