use learnatvcs::scrape;

use std::env;
use std::io;

static LEARNATVCS_USERNAME: &str = "LEARNATVCS_USERNAME";
static LEARNATVCS_PASSWORD: &str = "LEARNATVCS_PASSWORD";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        // .pretty()
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
        learnatvcs::TargetQuarter::All,
        learnatvcs::TargetDate::All,
    )
    .await?;
    println!("{}", serde_json::to_string(&output).unwrap());
    Ok(())
}
