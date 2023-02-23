use crate::datematcher::{ClassDay, Date};

use std::iter;
use std::str::FromStr;
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
impl TargetDate {
    /// Filters `link` based on `self`
    pub fn filter<'a>(
        &'a self,
        links: &'a Vec<(String, String)>,
        // XXX: We might do the NewType pattern later
    ) -> Box<dyn Iterator<Item = (String, String)> + 'a> {
        match self {
            TargetDate::Latest => {
                let (date_name, plan_url) = links.last().unwrap();
                return Box::new(iter::once((date_name.to_string(), plan_url.to_string())));
            }
            TargetDate::Selected(dates) => {
                let output = links
                    .iter()
                    // For every date, match the text with the given date
                    .filter(move |(date_name, _)| {
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
                    .cloned();
                return Box::new(output);
            }
            TargetDate::All => return Box::new(links.iter().cloned()),
        }
    }
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
