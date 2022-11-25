use crate::datematcher::{ClassDay, Date};
use crate::errors::LearnAtVcsError;
use futures::future::join_all;
use lazy_static::lazy_static;

use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time;

use std::str::FromStr;

/// A `Result` alias where the `Err` case is `errors::ScrapeError`.
pub type Result<T> = std::result::Result<T, LearnAtVcsError>;
/// Choose a date to scrape
#[derive(Clone, Debug)]
pub enum TargetDate {
    /// Scrape latest dates
    Latest,
    /// Scrape all dates
    All,
    /// Scrape selected dates
    Selected(Vec<Date>),
}
/// Closely follows learn@vcs's structure
pub type Output = HashMap<String, Option<HashMap<String, Option<String>>>>;
lazy_static! {
    static ref LOGIN_TOKEN_SELECTOR: Selector = Selector::parse(r#"[name="logintoken"]"#).unwrap();
    static ref CLASS_LIST_ITEM_SELECTOR: Selector = Selector::parse("ul.unlist > li a").unwrap();
    static ref LESSON_PLAN_LINK_SELECTOR: Selector =
        Selector::parse(r#"a[title^="Lesson"]"#).unwrap();
    static ref LESSON_PLAN_QUARTER_SELECTOR: Selector =
        Selector::parse(".activity.book.modtype_book a").unwrap();
    static ref LESSON_PLAN_QUARTER_TEXT_SELECTOR: Selector =
        Selector::parse("span.instance_name").unwrap();
    static ref LESSON_PLAN_DATE_SELECTOR: Selector =
        Selector::parse(".book_toc a,.book_toc strong").unwrap();
    static ref LESSON_PLAN_CONTENTS_SELECTOR: Selector =
        Selector::parse("section#region-main > div[role='main']").unwrap();
}

/// This is the URL of learn@vcs
pub const BASE_URL: &str = "https://learn.vcs.net";

#[tracing::instrument]
async fn scrape_plans(
    client: reqwest::Client,
    url: &str,
    dates: &TargetDate,
) -> Result<HashMap<String, Option<String>>> {
    // TODO: Remove unnecessary requests by reusing `lesson_plans` via Box::pin(future) and Box(dyn future)
    let tasks = {
        let lesson_plans = Html::parse_document(&fetch(&client, url).await?);
        let mut tasks = Vec::new();
        let links = lesson_plans.select(&LESSON_PLAN_DATE_SELECTOR);

        let mut add_task = |date_name: String, plan_url: String| {
            let client_clone = client.clone();
            tasks.push(tokio::spawn(async move {
                (date_name, scrape_plan(&client_clone, &plan_url).await.ok())
            }));
        };
        match dates {
            TargetDate::Latest => {
                let res = links.last().ok_or(LearnAtVcsError::StructureChanged)?;
                let date_name = res
                    .value()
                    .attr("title")
                    .or_else(|| res.text().next())
                    .ok_or(LearnAtVcsError::StructureChanged)?;
                let plan_url = res
                    .value()
                    .attr("href")
                    .map(|link| format!("{}/mod/book/{}", BASE_URL, link))
                    .or_else(|| Some(url.to_string()))
                    .unwrap();
                add_task(date_name.to_string(), plan_url);
            }
            remaining => {
                // For every date, match the text with the given date
                for res in links.filter(|element| match remaining {
                    TargetDate::Selected(dates) => {
                        let class_day = ClassDay::from_str(
                            element
                                .value()
                                .attr("title")
                                .or_else(|| element.text().next())
                                .unwrap(),
                        )
                        .unwrap();
                        for date in dates {
                            if class_day.matches(date) {
                                return true;
                            }
                        }
                        false
                    }
                    TargetDate::All => true,
                    _ => {
                        unreachable!()
                    }
                }) {
                    let date_name = res
                        .value()
                        .attr("title")
                        .or_else(|| res.text().next())
                        .ok_or(LearnAtVcsError::StructureChanged)?;
                    let plan_url = res
                        .value()
                        .attr("href")
                        .map(|link| format!("{}/mod/book/{}", BASE_URL, link))
                        .or_else(|| Some(url.to_string()))
                        .unwrap();
                    add_task(date_name.to_string(), plan_url);
                }
            }
        };
        tasks
    };
    tracing::info!("LEVEL 2: Per-date scrape loop");
    let mut output = HashMap::new();
    for (key, value) in (join_all(tasks).await).into_iter().flatten() {
        output.insert(key, value);
    }
    Ok(output)
}
#[tracing::instrument]
async fn fetch(client: &reqwest::Client, url: &str) -> Result<String> {
    tracing::info!("Fetching {url}");
    let now = time::Instant::now();
    let output = client
        .get(url)
        .send()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?
        .text()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?;
    tracing::info!(
        "Finished fetching {} (finished in {})",
        url,
        now.elapsed().as_micros()
    );
    Ok(output)
}
fn get_quarter_url(contents: &str, target_quarter: Option<usize>) -> Result<String> {
    let plan_quarters = Html::parse_document(contents);
    let quarter_element = match target_quarter {
        None => plan_quarters.select(&LESSON_PLAN_QUARTER_SELECTOR).last(),
        Some(quarter_num) => 'output: {
            // XXX: This is probably highly inefficient

            // For every link we have, find the link with the quarter number in its name
            for element in plan_quarters.select(&LESSON_PLAN_QUARTER_SELECTOR) {
                let Some(text_element) =
                        element.select(&LESSON_PLAN_QUARTER_TEXT_SELECTOR).next() else {continue};
                let Some(text_node) = text_element.text().next() else {continue};

                if text_node.contains(&quarter_num.to_string()) {
                    break 'output Some(element);
                }
            }
            None
        }
    }
    .ok_or(LearnAtVcsError::InvalidQuarter)?;
    Ok(quarter_element.value().attr("href").unwrap().to_owned())
}
/// Given the teacher's learn@vcs page, scrape lesson plans
#[tracing::instrument]
async fn scrape_page(
    client: reqwest::Client,
    url: &str,
    quarter: Option<usize>,
    dates: &TargetDate,
) -> Result<HashMap<String, Option<String>>> {
    let contents = fetch(&client, url).await?;
    let link = {
        // gets the link of the lesson plan tab
        // TODO: Cache
        Html::parse_document(&contents)
            .select(&LESSON_PLAN_LINK_SELECTOR)
            .next()
            .map(|element| element.value().attr("href").unwrap())
            .ok_or(LearnAtVcsError::NoLessonPlans)? // shouldn't happen
            .to_owned()
    };
    let learnatvcs_page_contents = fetch(&client, &link).await?;
    let quarter_url = get_quarter_url(&learnatvcs_page_contents, quarter)?;
    scrape_plans(client, &quarter_url, dates).await
}
#[tracing::instrument]
async fn scrape_plan(client: &reqwest::Client, url: &str) -> Result<String> {
    Html::parse_document(&fetch(client, url).await?)
        .select(&LESSON_PLAN_CONTENTS_SELECTOR)
        .next()
        .map(|element| element.inner_html())
        .ok_or(LearnAtVcsError::StructureChanged)
}
/// Given the contents of BASE_URL, return the links and names of classes
fn get_teacher_pages(contents: &str) -> Vec<(String, String)> {
    let dom = Html::parse_document(contents);
    let mut output = Vec::with_capacity(8);
    // Skip "VCJH iPad Program Home Page" and "VCJH Student Home Page"
    // We might remove this entirely and just ditch a class
    // when a class doesn't have a "lesson plan" tab or when
    // it's name doesn't have a "-"
    for class_link in dom.select(&CLASS_LIST_ITEM_SELECTOR).skip(2) {
        let name = class_link.value().attr("title").unwrap().to_string();
        // Otherwise it's not a class...
        if name.find('-').is_none() {
            break;
        }
        let url = class_link.value().attr("href").unwrap().to_owned();
        output.push((name, url));
    }
    output
}
/// Scrape all lesson plans
#[tracing::instrument]
pub async fn scrape(
    username: String,
    password: String,
    quarter: Option<usize>,
    dates: TargetDate,
) -> Result<Output> {
    tracing::info!("Initializing...");
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .map_err(LearnAtVcsError::ReqwestError)?;

    tracing::info!("Getting login token... ");

    let doc = client
        .get(format!("{}/login/index.php", BASE_URL))
        .send()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?
        .text()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?;
    let dom = Html::parse_document(&doc);
    // get login token from BASE_URL
    let login_token = dom
        .select(&LOGIN_TOKEN_SELECTOR)
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap()
        .to_string();
    tracing::info!("Login token: {login_token}");
    let mut auth = HashMap::new();
    auth.insert("username", username);
    auth.insert("password", password);
    auth.insert("logintoken", login_token);
    tracing::info!("Logging in... ");

    // Since authenticating also returns
    // the homepage, we might as well re-use that
    let cached_homepage = client
        .post(format!("{}/login/index.php", BASE_URL))
        .form(&auth)
        .send()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?
        .text()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?;
    tracing::info!("Finished logging in");

    let tasks = get_teacher_pages(&cached_homepage)
        .into_iter()
        .map(|x: (String, String)| {
            let (name, url) = x;
            let client_clone = client.clone();
            let dates_clone = dates.clone();
            tokio::spawn(async move {
                (
                    name,
                    scrape_page(client_clone, &url, quarter, &dates_clone)
                        .await
                        .ok(),
                )
            })
        });
    tracing::info!("LEVEL 1: Root scrape loop");
    let mut output = HashMap::new();
    for (key, value) in (join_all(tasks).await).into_iter().flatten() {
        output.insert(key, value);
    }
    Ok(output)
}
