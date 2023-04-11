use anyhow::Result;
use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser, Default)]
#[clap(about = "Login with a business")]
pub struct Options {
    #[clap(long, help = "Business Api Key, you can use `--key=` param to provide the key as well.")]
    pub key: Option<String>
}

pub async fn run(options: Options) -> Result<()> {
    let mut config = Config::load();

    if config.get_biz_id().is_some() {
        println!("You're already authenticated!");
    } else {
        if options.key.is_none() || options.key.as_ref().unwrap().is_empty() {
            println!("Redirecting you to https://dash.whop.com/settings/developer, where you can get an api key.\n Please put it below.");
            
            if let Err(e) = webbrowser::open("https://dash.whop.com/settings/developer") {
                println!("Failed to open browser: {}", e); 
            };
    
            let key = dialoguer::Input::new()
                .with_prompt("Api key")
                .interact()?;
    
            config.set_biz_id(key);
    
            config.save()?;
    
            println!("Successfully authenticated with your business!");
    
        } else {
            config.set_biz_id(options.key.unwrap());
    
            println!("Successfully authenticated with your business!");
        }
    }

    Ok(())
}