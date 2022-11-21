// TODO: optimize
#![warn(missing_docs)]
//! Fast request-based scraping API for learn@vcs
use std::pin::Pin;
/// Date models
pub mod datematcher;
/// Error types
pub mod errors;
use datematcher::{ClassDay, Date};
/// Choose a date to scrape
#[derive(Clone)]
pub enum TargetDate {
    /// Scrape latest dates
    Latest,
    /// Scrape all dates
    All,
    /// Scrape selected dates
    Selected(Vec<Date>),
}
use errors::LearnAtVcsError;
use std::str::FromStr;

/// A `Result` alias where the `Err` case is `errors::ScrapeError`.
pub type Result<T> = std::result::Result<T, LearnAtVcsError>;
use futures::future::join_all;
use lazy_static::lazy_static;
use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io::{self, Write};
use tokio;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
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
/// Represents a logged-in learn@vcs session
pub struct Session {
    client: reqwest::Client,
    cached_homepage: String,
    // request_cache: Arc<Mutex<HashMap<String, String>>>,
}

impl Session {
    /// Minimally create a logged-in session
    pub async fn new(username: String, password: String) -> Result<Self> {
        eprintln!("Initializing...");
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?;
        eprint!("Getting login token... ");
        io::stderr().flush().unwrap();
        let doc = client
            .get(format!("{}/login/index.php", BASE_URL))
            .send()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?
            .text()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?;
        let dom = Html::parse_document(doc.as_str());
        // get login token from BASE_URL
        let login_token = dom
            .select(&LOGIN_TOKEN_SELECTOR)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap()
            .to_string();
        eprintln!("done: {login_token}");
        let mut auth = HashMap::new();
        auth.insert("username", username);
        auth.insert("password", password);
        auth.insert("logintoken", login_token);
        eprint!("Logging in... ");
        io::stderr().flush().unwrap();
        // Since authenticating also returns
        // the homepage, we might as well re-use that
        let cached_homepage = client
            .post(format!("{}/login/index.php", BASE_URL))
            .form(&auth)
            .send()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?
            .text()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?;
        eprintln!("done");
        Ok(Session {
            client,
            cached_homepage,
            // request_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    async fn get_page(&self, url: &str) -> Result<String> {
        // let mut current_cache = self.request_cache.lock().await;
        // eprintln!("Got lock");
        // if current_cache.contains_key(url) {
        //     eprintln!("Cache hit");
        //     return Ok(current_cache.get(url).unwrap().to_string());
        // }
        eprintln!("Fetching {}", url);
        let output = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?
            .text()
            .await
            .map_err(|e| LearnAtVcsError::ReqwestError(e))?;
        // eprintln!("Done, inserting to cache");
        // (*current_cache).insert(url.to_string(), output.clone());
        Ok(output)
    }

    /// TODO: Scrape spec and multiple quarters and dates
    pub async fn scrape(
        // self: Pin<&Self>,
        &self,
        quarter: Option<usize>,
        dates: TargetDate,
    ) -> Result<Output> {
        let dom = Html::parse_document(self.cached_homepage.as_str());

        let mut tasks = Vec::new();
        let mut output = HashMap::new();
        // Skip "VCJH iPad Program Home Page" and "VCJH Student Home Page"
        // We might remove this entirely and just ditch a class
        // when a class doesn't have a "lesson plan" tab or when
        // it's name doesn't have a "-"
        for class_link in dom.select(&CLASS_LIST_ITEM_SELECTOR).skip(2) {
            let name = class_link.value().attr("title").unwrap().to_string();
            // Otherwise it's not a class...
            if name.find("-") == None {
                break;
            }

            let url = class_link.value().attr("href").unwrap().to_owned();
            eprintln!("Class: {}, url: {}", name, url);
            let dates_clone = dates.clone();

            // we do minimal parsing here
            // tasks.push(tokio::spawn(async move {
            //     (
            //         String::from(""),
            //         self.scrape_page("url", None, &TargetDate::All).await,
            //     )
            // }));
            tasks.push(tokio::spawn(async move {
                (
                    name,
                    self.scrape_page(&url, quarter, &dates_clone).await.ok(),
                )
            }));
        }
        eprintln!("LEVEL 1: Root scrape loop");
        // Concurrent; not parallel. May make it parallel in the future if it's even faster
        for res in join_all(tasks).await {
            if let Ok((key, value)) = res {
                output.insert(key, value);
            }
        }
        // Err(LearnAtVcsError::InvalidQuarter)
        Ok(output)
    }
    /// Given URL of learn@vcs page, find lesson plans
    async fn scrape_page(
        &self,
        url: &str,
        quarter: Option<usize>,
        dates: &TargetDate,
    ) -> Result<HashMap<String, Option<String>>> {
        let contents = self.get_page(url).await?;
        let link = {
            Html::parse_document(&contents)
                .select(&LESSON_PLAN_LINK_SELECTOR)
                .next()
                .map(|element| element.value().attr("href").unwrap())
                .ok_or(LearnAtVcsError::NoLessonPlans)? // shouldn't happen
                .to_owned()
        };
        let learnatvcs_page_contents = self.get_page(&link).await?;
        let quarter_url = {
            let plan_quarters = Html::parse_document(&learnatvcs_page_contents);
            let quarter_element = match quarter {
                None => plan_quarters.select(&LESSON_PLAN_QUARTER_SELECTOR).last(),
                Some(quarter_num) => 'output: {
                    // XXX: This is probably highly inefficient

                    // For every link we have, find the link with the quarter number in its name
                    for element in plan_quarters.select(&LESSON_PLAN_QUARTER_SELECTOR) {
                        let Some(text_element) =
                        element.select(&LESSON_PLAN_QUARTER_TEXT_SELECTOR).next() else {continue};
                        let Some(text_node) = text_element.text().next() else {continue};

                        if text_node.contains(&quarter_num.to_string()) {
                            eprintln!("Quarter found");
                            break 'output Some(element);
                        }
                    }
                    None
                }
            }
            .ok_or(LearnAtVcsError::InvalidQuarter)?;
            quarter_element.value().attr("href").unwrap().to_owned()
        };
        Ok(self.scrape_plans(&quarter_url, &dates).await?)
    }
    async fn scrape_plans(
        &self,
        url: &str,
        dates: &TargetDate,
    ) -> Result<HashMap<String, Option<String>>> {
        // TODO: Remove unnecessary requests by reusing `lesson_plans` via Box::pin(future) and Box(dyn future)
        let tasks = {
            let lesson_plans = Html::parse_document(&self.get_page(url).await?);
            let mut tasks = Vec::new();
            let links = lesson_plans.select(&LESSON_PLAN_DATE_SELECTOR);

            let mut add_task = |date_name: String, plan_url: String| {
                tasks.push(async move { (date_name, self.scrape_plan(&plan_url).await.ok()) });
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
                    eprintln!("Scraping for {date_name} (url: {plan_url})");
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
                                if class_day.matches(&date) {
                                    return true;
                                }
                            }
                            return false;
                        }
                        TargetDate::All => {
                            return true;
                        }
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
                        // eprintln!("Scraping for {date_name} (url: {plan_url})");
                        add_task(date_name.to_string(), plan_url);
                    }
                }
            };
            tasks
        };
        eprintln!("LEVEL 2: Per-date scrape loop");
        let mut output = HashMap::new();
        for (key, value) in join_all(tasks).await {
            // if let Ok((key, value)) = res {
            output.insert(key, value);
            // }
        }
        Ok(output)
    }

    async fn scrape_plan(&self, url: &str) -> Result<String> {
        // eprintln!("Parsing lesson plans for {url}");
        Html::parse_document(&self.get_page(url).await?)
            .select(&LESSON_PLAN_CONTENTS_SELECTOR)
            .next()
            .map(|element| element.inner_html())
            .ok_or(LearnAtVcsError::StructureChanged)
    }
}
#[cfg(test)]
mod tests {
    // use super::*;
}
