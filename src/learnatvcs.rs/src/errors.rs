use reqwest::Error as ReqwestError;
use thiserror::Error;
/// Contains all the errors the main scraper
/// could throw
#[derive(Error, Debug)]
pub enum LearnAtVcsError {
    /// No lesson plans could be found
    #[error("No lesson plans could be found")]
    NoLessonPlans,
    /// No lesson plans could be found
    #[error("No lesson plans could be found")]
    InvalidQuarter,
    /// Network error or errors related to the internal library
    #[error("Network error or errors related to the internal library")]
    ReqwestError(ReqwestError),
    /// Page structure had changed
    #[error("Page structure had changed")]
    StructureChanged,
}
/// All errors related to the
/// date model
#[derive(Error, Debug)]
pub enum DateError {
    /// Could not parse the input string
    /// into a date
    #[error("No date could be constructed")]
    ParseError,
    /// Invalid date. Perhaps the month or day
    /// values are out of range
    #[error("Invalid date")]
    InvalidDate,
}
