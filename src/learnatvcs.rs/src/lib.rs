// TODO: optimize
#![warn(missing_docs)]
//! Fast request-based scraping API for learn@vcs

/// Date models
pub mod datematcher;
pub use datematcher::Date;
/// Error types
pub mod errors;
mod scraper;
pub use crate::scraper::{scrape, TargetDate, TargetQuarter};
#[cfg(test)]
mod tests {
    // use super::*;
}
