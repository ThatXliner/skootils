use learnatvcs::Session;
use serde_json;
use std::env;
use tokio;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::new(
        env::var("LEARNATVCS_USERNAME").expect("Need"),
        env::var("LEARNATVCS_PASSWORD").expect("Need"),
    )
    .await?;
    let output = session.scrape(None).await?;
    println!("{}", serde_json::to_string(&output).unwrap());
    Ok(())
}
