mod browser_auth;

use anyhow::Result;
use clap::Parser;

use crate::{util::types::User, config::Config};

use self::browser_auth::browser_login;

#[derive(Debug, Parser, Default, PartialEq, Eq)]
#[clap(about = "Login to whop")]
pub struct Options {}

pub async fn run(_options: Options) -> Result<()> {
    let config = Config::load();
    
    if config.get_user().is_some() {
        println!("You're already authenticated! Run {} whoami to check your info!", crate::vars::CLI_NAME);
    } else {
        let token = browser_login().await?; // uses whop's OAuth to get users access token. required to be able to call https://api.whop.com/api/v2/me endpoints

        handle_token(token).await?;
    }

    Ok(())
}

async fn handle_token(token: String) -> Result<()> {
    let client = reqwest::Client::new();

    let res = client.get("https://api.whop.com/api/v2/me")
        .bearer_auth(token.clone())
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let username = json["username"].as_str().unwrap_or_default().to_string();
    let id = json["id"].as_str().unwrap_or_default().to_string();

    let user_data = User {
        id,
        username,
        access_token: token
    };

    let mut config = Config::load();

    config.set_user(user_data);

    config.save()?;

    println!("🚀 Successfully authenticated to whop!");

    Ok(())
}