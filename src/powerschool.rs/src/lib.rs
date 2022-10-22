use anyhow;
use futures::future::join_all;
use reqwest;
use std::collections::HashMap;
use tokio;
// use std::mem;
use tl;
// struct PowerSchool {
//     let url:String
// }
#[tokio::main]
pub async fn fetch(
    url: String,
    username: String,
    password: String,
    quarters: Option<Vec<String>>,
) -> anyhow::Result<String> {
    async fn _scrape_quarter(quarter: String) {}
    let client = reqwest::Client::new();
    let mut auth = HashMap::new();
    let home_url = format!("{}/guardian/home.html", url);
    auth.insert("account", username);
    auth.insert("pw", password);
    client
        .post(&home_url)
        .form(&auth)
        .send()
        .await?
        .error_for_status()?;
    let html = client.get(&home_url).send().await?.text().await?;
    let doc = tl::parse(html.as_str(), tl::ParserOptions::default())?;
    let home_table = doc
        .get_element_by_id("quickLookup")
        .ok_or_else(|| Err("PowerSchool is down".into()))?;

    if let Some(quarters) = quarters {
        let tasks = Vec::with_capacity(quarters.len());
        for quarter in quarters {
            tasks.push(_scrape_quarter(quarter));
        }
        // XXX: VS Tokio style .spawn and channel?
        join_all(tasks).await; // todo: do smth with value
    } else {
        // get latest
    }
    Ok("Hello there".into())
}
