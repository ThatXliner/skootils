use crate::errors::DateError;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
lazy_static! {
    /// 1. `amonth.aday/bmonth.bday`
    /// 2. `amonth.aday - bmonth.bday`
    /// 3. `amonth aday/bday`
    /// 4. `amonth aday/bmonth bday`
    /// 5. `amonthaday` (e.g. `WK#1 Jan4-6 (Zoom Jan5)` gets `Jan 4-6` and `Jan5`)
    /// 6. `amonth aday - bmonth bday`
    /// The regex can be found at https://regex101.com/r/HjMkar/3
    static ref CLASS_DAY_RE: Regex = Regex::new(
        r"(?:(?P<amonth>[a-zA-Z0-9]+)[. ]?(?P<aday>\d+))(?:(?:\s*(?:/|-| \-))\s*((?P<bmonthorday>(?:[a-zA-Z]+|[0-9]+))(?:[. ](?P<bday>\d+))?)?)?"
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
fn normalize_month(month: &str) -> Option<u8> {
    // Try numeric month first
    month.parse::<u8>().ok().or_else(|| {
        month
            .get(0..3)
            .and_then(|month| MONTH2INT.get(&month.to_lowercase()).copied())
    })
}
/// Represents a date on the calendar
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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
        let month =
            normalize_month(caps.name("month").unwrap().as_str()).ok_or(DateError::ParseError)?;
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
#[derive(Copy, Clone)]
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
        let parsed = CLASS_DAY_RE
            .captures_iter(s)
            .filter_map(|caps| parse_date_capture(caps).ok())
            .collect::<Vec<ClassDay>>();
        match parsed.len() {
            2 => {
                // 2 days... one of the is A day and one is actually B day
                Ok(ClassDay {
                    a: parsed[0].a,
                    b: Some(parsed[1].a),
                })
            }
            1 => Ok(parsed[0]),
            _ => Err(DateError::ParseError),
        }
    }
}

fn parse_date_capture(caps: regex::Captures) -> Result<ClassDay, DateError> {
    let a_month =
        normalize_month(caps.name("amonth").unwrap().as_str()).ok_or(DateError::ParseError)?;
    let a_day = caps
        .name("aday")
        .unwrap()
        .as_str()
        .parse::<u8>()
        .or(Err(DateError::ParseError))?;
    let b_day = caps.name("bmonthorday").and_then(|b_month| {
        caps.name("bday")
            // not using .map_or_else since this is more clear
            .map(|b_day| bmonth_and_bday(b_month, b_day))
            .unwrap_or_else(|| a_month_and_aday(a_month, b_month))
    });
    Ok(ClassDay {
        a: Date::new(a_month, a_day).unwrap(),
        b: b_day,
    })
}
fn a_month_and_aday(a_month: u8, b_month: regex::Match) -> Option<Date> {
    // It's actually a day
    b_month
        .as_str()
        .parse::<u8>()
        // Examples of cases that return None: January 4 - U7
        .ok()
        .map(|b_day| Date::new(a_month, b_day).expect("Invalid date"))
}
fn bmonth_and_bday(b_month: regex::Match, b_day: regex::Match) -> Option<Date> {
    normalize_month(b_month.as_str()).and_then(|b_month| {
        Some(
            Date::new(
                b_month,
                // XXX: Don't fail silently?
                match b_day.as_str().parse::<u8>() {
                    Ok(it) => it,
                    Err(_) => return None,
                },
            )
            .expect("Invalid date"),
        )
    })
    // Examples of None: January 4 - Intro, February 7 - Unit 2, etc
}
