use serde::{Deserialize, Serialize};

use crate::datematcher::{ClassDay, Date};

use std::iter;
use std::str::FromStr;
use std::sync::Arc;
/// Choose a date to scrape
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "dates")]
pub enum TargetDate {
    /// Scrape latest dates
    Latest,
    /// Scrape all dates
    All,
    /// Scrape selected dates
    Selected(Arc<Vec<Date>>),
}
impl TargetDate {
    /// Filters `link` based on `self`
    pub fn filter<'a>(
        &'a self,
        links: &'a [(String, String)],
        // XXX: We might do the NewType pattern later
    ) -> Box<dyn Iterator<Item = (String, String)> + 'a> {
        match self {
            TargetDate::Latest => {
                let (date_name, plan_url) = links.last().unwrap();
                Box::new(iter::once((date_name.to_string(), plan_url.to_string())))
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
                        for date in (*dates).iter() {
                            if class_day.matches(date) {
                                return true;
                            }
                        }
                        false
                    })
                    .cloned();
                Box::new(output)
            }
            TargetDate::All => return Box::new(links.iter().cloned()),
        }
    }
}
/// Choose a quarter to scrape
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy)]
#[serde(tag = "type", content = "quarter")]
pub enum TargetQuarter {
    /// Scrape latest quarter
    Latest,
    /// Scrape all quarters
    All,
    /// Scrape selected quarters
    Selected(u8),
}
/// Shorthand for `TargetDates::Selected(Arc::new(vec![Date::new(...)...]));`
/// ```
/// # use std::sync::Arc;
/// # use learnatvcs::{TargetDate, Date, dates};
/// assert!(TargetDate::Selected(Arc::new(vec![Date::new(1, 20).unwrap()])) == dates!(1/20));
/// ```
#[macro_export]
macro_rules! dates {
    ($($month:literal / $day:literal),*) => {
       {
            use std::sync::Arc;
            $crate::TargetDate::Selected(Arc::new(vec![$($crate::Date::new($month, $day).unwrap(),)*]))
        }
    };
}
/// Shorthand for `TargetQuarter::Selected(Arc::new(vec![...]));`
/// ```
/// # use std::sync::Arc;
/// # use learnatvcs::{TargetQuarter, quarters};
/// assert!(TargetQuarter::Selected(1) == quarters!(1));
/// assert!(TargetQuarter::Selected(1 | 1 << 2) == quarters!(1, 3));
/// ```
#[macro_export]
macro_rules! quarters {
    ($($x:expr),*) => {
        {
            use std::sync::Arc;
            $crate::TargetQuarter::Selected(0_u8$( | 1 << ($x - 1))*)
        }
    };
}
