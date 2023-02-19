use crate::datematcher::{ClassDay, Date};
use crate::errors::LearnAtVcsError;
use lazy_static::lazy_static;
use tokio::task::JoinSet;

use scraper::{Html, Selector};
use std::collections::HashMap;

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
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();
        if links.len() == 0 {
            return Ok(HashMap::new());
            // return Err(LearnAtVcsError::NoLessonPlans);
        }

        let mut add_task = |date_name: String, plan_url: String| {
            let client_clone = client.clone();
            tasks.spawn(
                async move { (date_name, scrape_plan(&client_clone, &plan_url).await.ok()) },
            );
        };
        match dates {
            TargetDate::Latest => {
                let (date_name, plan_url) = links.last().unwrap();
                add_task(date_name.to_string(), plan_url.to_string());
            }
            remaining => {
                // For every date, match the text with the given date
                links
                    .iter()
                    .filter(|(date_name, _)| match remaining {
                        TargetDate::Selected(dates) => {
                            let class_day = ClassDay::from_str(date_name).unwrap();
                            for date in dates {
                                if class_day.matches(date) {
                                    return true;
                                }
                            }
                            // TODO: If date doesn't exist, don't fail silently?
                            false
                        }
                        TargetDate::All => true,
                        _ => {
                            unreachable!()
                        }
                    })
                    .for_each(|(date_name, plan_url)| {
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
    let output = client
        .get(url)
        .send()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?
        .text()
        .await
        .map_err(LearnAtVcsError::ReqwestError)?;
    Ok(output)
}
fn get_quarter_url(contents: &str, target_quarter: TargetQuarter) -> Vec<Option<String>> {
    let quarters = Html::parse_document(contents);
    let quarters = quarters // shadowed on purpose
        .select(&QUARTER_SELECTOR)
        .filter_map(|element| {
            let Some(text_element) =
                        element.select(&QUARTER_TEXT_SELECTOR).next() else {return None};
            let Some(text_node) = text_element.text().next() else {return None};

            if text_node.contains("Quarter") {
                Some((text_node, element.value().attr("href")))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    match target_quarter {
        TargetQuarter::Latest => vec![quarters
            .last()
            .and_then(|(_, link)| link.map(|x| x.to_owned()))],
        TargetQuarter::Selected(quarter_nums) => quarters
            .iter()
            .filter_map(|(text, link)| {
                for quarter_num in &quarter_nums {
                    if text.contains(&quarter_num.to_string()) {
                        return Some(link.map(|x| x.to_owned()));
                    }
                }
                return None;
            })
            .collect::<Vec<_>>(),
        TargetQuarter::All => quarters
            .iter()
            .map(|(_, link)| link.map(|x| x.to_owned()))
            .collect::<Vec<_>>(),
    }
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
            .map(|element| element.value().attr("href").unwrap())
            .ok_or(LearnAtVcsError::NoLessonPlans)? // shouldn't happen, except for Ms. Arild
            .to_owned()
    };
    let contents = fetch(&client, &link).await?; // shadow on purpose
    match &quarter {
        TargetQuarter::Latest => {
            let quarter_url = get_quarter_url(&contents, quarter.clone());
            let x = quarter_url[0]
                .as_ref()
                .ok_or(LearnAtVcsError::NoLessonPlans)?;
            Ok(HashMap::from([(
                String::from("latest"),
                Some(scrape_plans(client, x, dates).await.expect(url)),
            )]))
        }
        _ => todo!(), // TargetQuarter::All => {
                      //     let quarter_urls = get_quarter_url(&learnatvcs_page_contents, quarter.clone());
                      //     let mut output = HashMap::new();
                      //     let x = (quarter_urls.get(0).ok_or(LearnAtVcsError::InvalidQuarter)?)
                      //         .as_ref()
                      //         .ok_or(LearnAtVcsError::NoLessonPlans)?;

                      //     output.insert(
                      //         String::from("all"),
                      //         Some(scrape_plans(client, x, dates).await?),
                      //     );
                      //     Ok(output)
                      // }
                      // quarter => {
                      //     let quarter_urls = get_quarter_url(&learnatvcs_page_contents, quarter.clone());
                      //     let mut output = HashMap::new();
                      //     let x = (quarter_urls.get(0).ok_or(LearnAtVcsError::InvalidQuarter)?)
                      //         .as_ref()
                      //         .ok_or(LearnAtVcsError::NoLessonPlans)?;

                      //     output.insert(
                      //         String::from("all"),
                      //         Some(scrape_plans(client, x, dates).await?),
                      //     );
                      //     Ok(output)
                      // }
    }
}
#[tracing::instrument]
async fn scrape_plan(client: &reqwest::Client, url: &str) -> Result<String> {
    let output = Html::parse_document(&fetch(client, url).await?)
        .select(&LESSON_PLAN_CONTENTS_SELECTOR)
        .next()
        .map(|element| element.inner_html())
        .ok_or(LearnAtVcsError::StructureChanged);
    output
}
/// Given the contents of BASE_URL, return the links and names of classes
fn get_teacher_pages(contents: &str) -> Vec<(String, String)> {
    let dom = Html::parse_document(contents);
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
        let client_clone = client.clone();
        let dates_clone = dates.clone();
        let quarter_clone = quarter.clone();
        tasks.spawn(async move {
            (
                name,
                scrape_page(client_clone, &url, quarter_clone, &dates_clone)
                    .await
                    .ok(),
            )
        });
    }
    let mut output = HashMap::new();
    while let Some(handle) = tasks.join_next().await {
        let (key, value) = handle.expect("Join failed");
        output.insert(key, value);
    }
    Ok(output)
}
