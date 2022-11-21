use learnatvcs::scrape;
use serde_json;
use std::env;
use tokio;
static LEARNATVCS_USERNAME: &str = "LEARNATVCS_USERNAME";
static LEARNATVCS_PASSWORD: &str = "LEARNATVCS_PASSWORD";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = scrape(
        env::var(LEARNATVCS_USERNAME).expect("Need"),
        env::var(LEARNATVCS_PASSWORD).expect("Need"),
        None,
        learnatvcs::TargetDate::All,
    )
    .await?;
    println!("{}", serde_json::to_string(&output).unwrap());
    Ok(())
}
