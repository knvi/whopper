use anyhow::Result;
use clap::Parser;
use tabled::Table;

use crate::util::types::Membership;

#[derive(Debug, Parser)]
#[clap(about = "List user's memberships")]
pub struct Options {}

pub async fn run(_command: Options) -> Result<()> {

    let products: Vec<Membership> = crate::api::list::<Membership>(None, String::from("memberships")).await?;

    if products.is_empty() {
        println!("You don't have any memberships!");
        return Ok(());
    }
    
    let table = Table::new(products).to_string();

    println!("{}", table);

    Ok(())
}