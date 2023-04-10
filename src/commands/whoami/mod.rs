use anyhow::Result;
use clap::Parser;

use crate::{config::Config, util::types::User};

#[derive(Debug, Parser)]
#[clap(about = "Get data about your authentication")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {
    let config = Config::load();
    
    let user: User = match config.get_user() {
        Some(user) => user.to_owned(),
        None => {
            println!("You're not authenticated! Run {} auth login to authenticate!", crate::vars::CLI_NAME);
            return Ok(());
        }
    };
    
    println!("Youre currently authenticated as @{}, with your user id being {}", user.username, user.id);

    Ok(())
}