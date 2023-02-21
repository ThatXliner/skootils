use crate::datematcher::{ClassDay, Date};
use crate::errors::LearnAtVcsError;
use async_recursion::async_recursion;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use scraper::{Html, Selector};
use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

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
/// Choose a quarter to scrape
#[derive(Clone, Debug)]
pub enum TargetQuarter {
    /// Scrape latest quarter
    Latest,
    /// Scrape all quarters
    All,
    /// Scrape selected quarters
    Selected(Vec<usize>),
}
/// Closely follows learn@vcs's structure
/// Class, Quarter, Date, Contents
pub type Output = HashMap<String, Option<TeacherPage>>;
pub type TeacherPage = HashMap<String, Option<QuarterOutput>>;
pub type QuarterOutput = HashMap<String, Option<String>>;
lazy_static! {
    static ref LOGIN_TOKEN_SELECTOR: Selector = Selector::parse(r#"[name="logintoken"]"#).unwrap();
    static ref CLASS_LIST_ITEM_SELECTOR: Selector = Selector::parse("ul.unlist > li a").unwrap();
    static ref LESSON_PLAN_LINK_SELECTOR: Selector =
        Selector::parse(r#"a[title^="Lesson"]"#).unwrap();
    static ref QUARTER_SELECTOR: Selector = Selector::parse(".aalink").unwrap();
    static ref QUARTER_TEXT_SELECTOR: Selector = Selector::parse("span.instancename").unwrap();
    static ref LESSON_PLAN_DATE_SELECTOR: Selector =
        Selector::parse(".book_toc a,.book_toc strong").unwrap();
    static ref LESSON_PLAN_CONTENTS_SELECTOR: Selector = Selector::parse("[role='main']").unwrap();
    static ref QUARTER_NAME_RE: Regex = Regex::new(r"Quarter \d").unwrap();
}

/// This is the URL of learn@vcs
pub const BASE_URL: &str = "https://learn.vcs.net";
// Within the lesson plan book page, scrape the selected dates
#[tracing::instrument]
async fn scrape_plans(
    client: reqwest::Client,
    url: &str,
    dates: &TargetDate,
) -> Result<HashMap<String, Option<String>>> {
    // TODO: Remove unnecessary requests by reusing `lesson_plans` via Box::pin(future) and Box(dyn future)
    let mut tasks = {
        let lesson_plans = Html::parse_document(&fetch(&client, url).await?);
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
            // return Err(LearnAtVcsError::NoLessonPlans);
        }

        let mut add_task = |date_name: String, plan_url: String| {
            let client = client.clone();
            tasks.spawn(async move { (date_name, scrape_plan(&client, &plan_url).await.ok()) });
        };
        match dates {
            TargetDate::Latest => {
                let (date_name, plan_url) = links.last().unwrap();
                add_task(date_name.to_string(), plan_url.to_string());
            }
            TargetDate::Selected(dates) => {
                links
                    .iter()
                    // For every date, match the text with the given date
                    .filter(|(date_name, _)| {
                        let Ok(class_day) = ClassDay::from_str(date_name) else {
                            // TODO: Yearbook has its dates like `Quarter 3`
                            tracing::warn!("Could not parse {date_name}");
                            return false
                        };
                        for date in dates {
                            if class_day.matches(date) {
                                return true;
                            }
                        }
                        false
                    })
                    .for_each(|(date_name, plan_url)| {
                        add_task(date_name.to_string(), plan_url.to_string())
                    });
            }
            TargetDate::All => {
                links.iter().for_each(|(date_name, plan_url)| {
                    add_task(date_name.to_string(), plan_url.to_string())
                });
            }
        };
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
fn get_quarter_urls(contents: &str) -> HashMap<String, String> {
    let quarters = Html::parse_document(contents);
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
    quarters
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
    let contents = fetch(&client, url).await?;
    let link = {
        // gets the link of the lesson plan tab
        // TODO: Cache
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
    let contents = fetch(&client, &link).await?; // shadowing on purpose

    let quarter_urls = get_quarter_urls(&contents);
    let filtered_quarters: HashMap<String, String> = match quarter {
        TargetQuarter::Latest => {
            let mut output = HashMap::with_capacity(1);
            let mut test_quarter = 4;
            while test_quarter > 0 {
                let name = format!("Quarter {test_quarter}");
                match quarter_urls.get(&name) {
                    Some(url) => {
                        output.insert("latest".into(), url.into());
                        break;
                    }
                    None => test_quarter -= 1,
                }
            }
            output
        }
        TargetQuarter::Selected(quarter_nums) => {
            let mut output = HashMap::with_capacity(quarter_nums.len());
            for quarter_num in quarter_nums {
                let name = format!("Quarter {quarter_num}");
                if let Some(url) = quarter_urls.get(&name) {
                    output.insert(name, url.into());
                }
            }
            output
        }
        TargetQuarter::All => quarter_urls,
    };
    let mut tasks = JoinSet::new();
    for (name, quarter_url) in filtered_quarters.iter() {
        let name = name.to_string().clone();
        let client = client.clone();
        let quarter_url = quarter_url.clone();
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
#[tracing::instrument]
async fn scrape_plan(client: &reqwest::Client, url: &str) -> Result<String> {
    Ok(Html::parse_document(&fetch(client, url).await?)
        .select(&LESSON_PLAN_CONTENTS_SELECTOR)
        .next()
        .map(|element| element.inner_html())
        .expect("Oh no, we need to fix the scrapers"))
}
/// Given the contents of BASE_URL, return the links and names of classes
fn get_teacher_pages(contents: &str) -> Vec<(String, String)> {
    let dom = Html::parse_document(contents);
    // There's at least 8 classes
    let mut output = Vec::with_capacity(8);
    // We might remove this entirely and just ditch a class
    // when a class doesn't have a "lesson plan" tab or when
    // it's name doesn't have a "-"
    for class_link in dom.select(&CLASS_LIST_ITEM_SELECTOR) {
        let name = class_link.value().attr("title").unwrap().to_string();

        // Otherwise it's not a class...
        if !name.contains('-') {
            continue;
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
    quarter: TargetQuarter,
    dates: TargetDate,
) -> Result<Output> {
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
    for (name, url) in get_teacher_pages(&cached_homepage) {
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
