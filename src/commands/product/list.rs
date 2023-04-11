use std::env;

use anyhow::Result;
use clap::Parser;
use tabled::{Tabled, Table};

use crate::{config::Config, util::types::Product};

#[derive(Debug, Parser)]
#[clap(about = "List business' products")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {
    let config = Config::load();
    
    // let biz_id = match config.get_biz_id() {
    //     Some(biz_id) => biz_id.to_owned(),
    //     None => {
    //         println!("No business authenticated. Use `{} biz init` to login with a business.", crate::vars::CLI_NAME);
    //         return Ok(());
    //     }
    // };

    let products: Vec<Product> = crate::api::product::list(env::var("WHOP_API_KEY").expect("Please set the WHOP_API_KEY environment variable.")).await?;
    
    let table = Table::new(products).to_string();

    println!("{}", table);

    Ok(())
}