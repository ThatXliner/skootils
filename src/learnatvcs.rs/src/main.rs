use learnatvcs::quarters;
use learnatvcs::scrape;

use std::env;
use std::fs;
use std::io;
use tracing::Level;
static LEARNATVCS_USERNAME: &str = "LEARNATVCS_USERNAME";
static LEARNATVCS_PASSWORD: &str = "LEARNATVCS_PASSWORD";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        // .pretty()
        .with_max_level(Level::INFO)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Send to standard error
        .with_writer(io::stderr)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let output = scrape(
        env::var(LEARNATVCS_USERNAME).expect("Need"),
        env::var(LEARNATVCS_PASSWORD).expect("Need"),
        learnatvcs::quarters!(3),
        // quarters!(1, 2),
        learnatvcs::TargetDate::Latest,
        // learnatvcs::dates![1 / 20],
    )
    .await?;
    let data = serde_json::to_string(&output)?;
    fs::write("output.json", data).expect("msg");
    Ok(())
}
