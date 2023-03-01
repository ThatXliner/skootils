#![warn(missing_docs)]
//! Fast request-based scraping API for learn@vcs

/// Date models
pub mod datematcher;
pub use datematcher::Date;
/// Error types
pub mod errors;
pub use crate::models::{TargetDate, TargetQuarter};
mod models;
mod scraper;
mod utils;
pub use crate::scraper::scrape;
#[cfg(test)]
mod tests {
    // use super::*;
}
