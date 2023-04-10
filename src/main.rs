use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    whopper::run().await?;

    Ok(())
}