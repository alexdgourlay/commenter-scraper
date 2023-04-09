//! A library for scraping valuable content from HTML documents.
//!
//! Example
//! ```rust
//! use content_scraper::ContentScraper;
//! 
//! let content_scraper = ContentScraper::new(r#"<html></html>");
//! ```
mod content_scraper;
pub use content_scraper::ContentScraper;