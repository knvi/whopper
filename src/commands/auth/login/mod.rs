mod browser_auth;

use anyhow::Result;
use clap::Parser;

use self::browser_auth::browser_login;

#[derive(Debug, Parser, Default, PartialEq, Eq)]
#[clap(about = "Login to whop")]
pub struct Options {
    #[clap(long = "token", help = "Whop Account Token")]
    pub token: Option<String>,
    #[clap(long = "email", help = "Whop Account Email")]
    pub email: Option<String>,
    #[clap(long = "password", help = "Whop Account Password")]
    pub password: Option<String>,
}

pub async fn run(options: Options) -> Result<()> {
    // TODO: implement

    let token = browser_login().await?;

    Ok(())
}