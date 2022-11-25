use crate::errors::DateError;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
lazy_static! {
    static ref CLASS_DAY_RE: Regex = Regex::new(
        r"(?P<amonth>[a-zA-Z]+) (?P<aday>\d+)(?:/(?P<bmonth>[a-zA-Z]+)?[ ]?(?P<bday>\d+))?"
    )
    .unwrap();
    static ref DATE_RE: Regex = Regex::new(r"(?P<month>[a-zA-Z]+) (?P<day>\d+)").unwrap();
    static ref MONTH2INT: HashMap<String, u8> = HashMap::from([
        ("jan".into(), 1),
        ("feb".into(), 2),
        ("mar".into(), 3),
        ("apr".into(), 4),
        ("may".into(), 5),
        ("jun".into(), 6),
        ("jul".into(), 7),
        ("aug".into(), 8),
        ("sep".into(), 9),
        ("oct".into(), 10),
        ("nov".into(), 11),
        ("dec".into(), 12)
    ]);
}
fn validate(month: u8, day: u8) -> Result<(), DateError> {
    if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
        return Err(DateError::InvalidDate);
    }
    Ok(())
}
fn normalize_month(month: &str) -> u8 {
    *(MONTH2INT.get(&month[0..3].to_lowercase()).unwrap())
}
/// Represents a date on the calendar
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Date {
    month: u8,
    day: u8,
}
impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.month == other.month && self.day == other.day
    }
}
impl Date {
    /// Construct a Date model
    pub fn new(month: u8, day: u8) -> Result<Self, DateError> {
        validate(month, day)?;
        Ok(Self { month, day })
    }
}
impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{} {}", &self.month, &self.day)
    }
}
impl FromStr for Date {
    type Err = DateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = DATE_RE.captures(s).ok_or(DateError::ParseError)?;
        // normalize month
        let month = normalize_month(caps.name("month").unwrap().as_str());
        // parse day into integer
        let day = caps
            .name("day")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .or(Err(DateError::ParseError))?;
        validate(month, day)?;
        Ok(Date { month, day })
    }
}

/// Represents a date on the lesson plans
pub struct ClassDay {
    /// The A day, or just the primary day
    a: Date,
    /// The B day, if known
    b: Option<Date>,
}

impl ClassDay {
    /// Constructs a ClassDay
    pub fn new(a: Date, b: Option<Date>) -> Self {
        Self { a, b }
    }
    /// Matches a `Date` against the `ClassDay`
    pub fn matches(&self, other: &Date) -> bool {
        if let Some(b_day) = &self.b {
            *other == self.a || *other == *b_day
        } else {
            *other == self.a
        }
    }
}
impl PartialEq for ClassDay {
    fn eq(&self, other: &ClassDay) -> bool {
        other.a == self.a || other.b == self.b
    }
}
impl FromStr for ClassDay {
    type Err = DateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = CLASS_DAY_RE.captures(s).ok_or(DateError::ParseError)?;
        let a_month = normalize_month(caps.name("amonth").unwrap().as_str());
        let a_day = caps
            .name("aday")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .or(Err(DateError::ParseError))?;
        Ok(ClassDay {
            a: Date::new(a_month, a_day)?,
            b: caps
                .name("bday")
                .and_then(|day| day.as_str().parse::<u8>().ok())
                .and_then(|b_day| {
                    let b_month = caps
                        .name("bmonth")
                        .map(|m| normalize_month(m.as_str()))
                        .unwrap_or(a_month);
                    Date::new(b_month, b_day).ok()
                }),
        })
    }
}
