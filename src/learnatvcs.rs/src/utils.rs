// Synchronous utility functions

use crate::models::TargetQuarter;
use lazy_static::lazy_static;
use std::collections::HashMap;

use scraper::{Html, Selector};

pub(crate) fn filter_quarter_urls(
    quarter_urls: HashMap<String, String>,
    quarter: TargetQuarter,
) -> HashMap<String, String> {
    match quarter {
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
    }
}

pub(crate) fn parse_plan(contents: String) -> String {
    lazy_static! {
        static ref LESSON_PLAN_CONTENTS_SELECTOR: Selector =
            Selector::parse("[role='main']").unwrap();
    }
    Html::parse_document(&contents)
        .select(&LESSON_PLAN_CONTENTS_SELECTOR)
        .next()
        .map(|element| element.inner_html())
        .expect("Oh no, we need to fix the scrapers")
}
/// Given the contents of BASE_URL, return the links and names of classes
pub(crate) fn get_teacher_pages(contents: &str) -> Vec<(String, String)> {
    lazy_static! {
        static ref CLASS_LIST_ITEM_SELECTOR: Selector =
            Selector::parse("ul.unlist > li a").unwrap();
    }
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
