use crate::errors::LearnAtVcsError;
use crate::models::{TargetDate, TargetQuarter};
use crate::utils;
use async_recursion::async_recursion;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use scraper::{Html, Selector};
use std::cmp::max;
use std::collections::HashMap;

use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

/// A `Result` alias where the `Err` case is `errors::ScrapeError`.
pub type Result<T> = std::result::Result<T, LearnAtVcsError>;

/// Closely follows learn@vcs's structure
/// Class, Quarter, Date, Contents
pub type Output = HashMap<String, Option<TeacherPage>>;
pub type TeacherPage = HashMap<String, Option<QuarterOutput>>;
pub type QuarterOutput = HashMap<String, Option<String>>;

/// This is the URL of learn@vcs
pub const BASE_URL: &str = "https://learn.vcs.net";

// Within the lesson plan book page, scrape the selected dates
#[tracing::instrument]
async fn scrape_plans(
    client: reqwest::Client,
    url: &str,
    dates: &TargetDate,
) -> Result<HashMap<String, Option<String>>> {
    lazy_static! {
        static ref LESSON_PLAN_DATE_SELECTOR: Selector =
            Selector::parse(".book_toc a,.book_toc strong,.menuwrapper a").unwrap();
    }
    let mut tasks = {
        let contents = fetch(&client, url).await?;
        let lesson_plans = Html::parse_document(&contents);
        let mut tasks = JoinSet::new();
        let links = lesson_plans
            .select(&LESSON_PLAN_DATE_SELECTOR)
            .map(|link| {
                (
                    link.value()
                        .attr("title")
                        .map(|x| x.to_string())
                        .or_else(|| link.text().next().map(|x| x.to_string()))
                        .expect("This should never happen"), // XXX? use StructureChanged?
                    link.value()
                        .attr("href")
                        .map(|link| format!("{BASE_URL}/mod/book/{link}"))
                        .or_else(|| Some(url.to_string()))
                        .expect("This should never happen"),
                )
            })
            .collect::<Vec<_>>();
        if links.is_empty() {
            return Ok(HashMap::new());
        }
        dates
            .filter(&links)
            .for_each(|(date_name, plan_url): (String, String)| {
                let client = client.clone();
                if plan_url == url {
                    let contents = contents.clone();
                    tasks.spawn(async move { (date_name, Some(utils::parse_plan(contents))) });
                } else {
                    tasks.spawn(async move {
                        (
                            date_name,
                            fetch(&client, &plan_url).await.map(utils::parse_plan).ok(),
                        )
                    });
                }
            });

        tasks
    };
    let mut output = HashMap::new();
    while let Some(handle) = tasks.join_next().await {
        let (key, value) = handle.expect("Join failed");
        output.insert(key, value);
    }
    Ok(output)
}
#[tracing::instrument]
async fn fetch(client: &reqwest::Client, url: &str) -> Result<String> {
    const MAX_RETRIES: u8 = 5;
    const BACKOFF_BASE: u32 = 500;
    const JITTER_PERCENT: f32 = 0.3; // 30%
    #[async_recursion]
    async fn _fetch(client: &reqwest::Client, url: &str, retries_left: u8) -> Result<String> {
        match client
            .get(url)
            .send()
            .await
            .map_err(LearnAtVcsError::ReqwestError)?
            .error_for_status()
        {
            Err(_res) => {
                if retries_left == 0 {
                    Err(LearnAtVcsError::ReqwestError(_res))
                } else {
                    tracing::warn!(
                        "{url} errored, exponential backoff with {retries_left} retries left"
                    );

                    let delay = {
                        let base_delay: u64 =
                            BACKOFF_BASE.pow((MAX_RETRIES - retries_left).into()).into();
                        let jitter_amount: u64 = (base_delay as f32 * JITTER_PERCENT) as u64;
                        thread_rng().gen_range(
                            max(0, base_delay - jitter_amount)..=base_delay + jitter_amount,
                        )
                    };
                    // add jitter
                    sleep(Duration::from_millis(delay)).await;
                    _fetch(client, url, retries_left - 1).await
                }
            }
            Ok(res) => Ok(res.text().await.map_err(LearnAtVcsError::ReqwestError)?),
        }
    }
    _fetch(client, url, MAX_RETRIES).await
}
async fn get_quarter_urls(client: &reqwest::Client, url: &str) -> Result<HashMap<String, String>> {
    lazy_static! {
        static ref LESSON_PLAN_LINK_SELECTOR: Selector =
            Selector::parse(r#"a[title*="Lesson"]"#).unwrap();
        static ref QUARTER_SELECTOR: Selector = Selector::parse(".aalink").unwrap();
        static ref QUARTER_TEXT_SELECTOR: Selector = Selector::parse("span.instancename").unwrap();
        static ref QUARTER_NAME_RE: Regex = Regex::new(r"Quarter \d").unwrap();
    }
    // TODO: Cache this function
    let contents = fetch(client, url).await?;
    let link = {
        // gets the link of the lesson plan tab
        Html::parse_document(&contents)
            .select(&LESSON_PLAN_LINK_SELECTOR)
            .next()
            .ok_or(LearnAtVcsError::NoLessonPlans)? // shouldn't happen, except for Ms. Arild
            // We may already be on the lesson plan page
            .value()
            .attr("href")
            .unwrap_or(url) // actually, an .unwrap would suffice
            .to_owned()
    };
    let contents = fetch(client, &link).await?;
    let quarters = Html::parse_document(&contents);
    let quarters = quarters // shadowed on purpose
        .select(&QUARTER_SELECTOR)
        .filter_map(|element| {
            let Some(text_element) =
                        element.select(&QUARTER_TEXT_SELECTOR).next() else {return None};
            let Some(text_node) = text_element.text().next() else {return None};
            QUARTER_NAME_RE.find(text_node).map(|captures| {
                (
                    captures.as_str().into(),
                    element.value().attr("href").unwrap().into(),
                )
            })
        })
        .collect::<_>();
    Ok(quarters)
}

/// Given the teacher's learn@vcs page, scrape lesson plans
#[tracing::instrument]
async fn scrape_page(
    client: reqwest::Client,
    url: &str,
    quarter: TargetQuarter,
    dates: &TargetDate,
    // quarter: date: contents
) -> Result<TeacherPage> {
    let quarter_urls = get_quarter_urls(&client, url).await?;

    let filtered_quarters: HashMap<String, String> =
        utils::filter_quarter_urls(quarter_urls, quarter);
    let mut tasks = JoinSet::new();
    for (name, quarter_url) in filtered_quarters {
        let client = client.clone();
        let dates = dates.clone();

        tasks.spawn(async move { (name, scrape_plans(client, &quarter_url, &dates).await.ok()) });
    }

    let mut output = HashMap::new();
    while let Some(handle) = tasks.join_next().await {
        let (key, value) = handle.expect("Join failed");
        output.insert(key, value);
    }
    Ok(output)
}

/// Scrape all lesson plans
#[tracing::instrument]
pub async fn scrape(
    username: String,
    password: String,
    quarter: TargetQuarter,
    dates: TargetDate,
) -> Result<Output> {
    lazy_static! {
        static ref LOGIN_TOKEN_SELECTOR: Selector =
            Selector::parse(r#"[name="logintoken"]"#).unwrap();
    }
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .map_err(LearnAtVcsError::ReqwestError)?;

    let doc = client
        .get(format!("{BASE_URL}/login/index.php"))
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
    let mut auth = HashMap::new();
    auth.insert("username", username);
    auth.insert("password", password);
    auth.insert("logintoken", login_token);

    // Since authenticating also returns
    // the homepage, we might as well re-use that
    let cached_homepage = client
        .post(format!("{BASE_URL}/login/index.php"))
        .form(&auth)
        .send()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?
        .text()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?;

    let mut tasks = JoinSet::new();
    for (name, url) in utils::get_teacher_pages(&cached_homepage) {
        let client = client.clone();
        let dates = dates.clone();
        let quarter = quarter.clone();
        tasks.spawn(async move { (name, scrape_page(client, &url, quarter, &dates).await.ok()) });
    }
    let mut output = HashMap::new();
    while let Some(handle) = tasks.join_next().await {
        let (key, value) = handle.expect("Join failed");
        output.insert(key, value);
    }
    Ok(output)
}
