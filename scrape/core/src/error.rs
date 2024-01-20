use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScrapeError<'a> {
    #[error("A CSS Selector in the '{0}' scraper failed")]
    CSSSelectorFailed(&'a str),
    #[error("The HTML structure assumed in the '{0}' scraper failed")]
    InvalidStructureAssumed(&'a str),
    #[error("Failed to parse a scraped string value in the '{0}' scraper")]
    FailedToParseStringValue(&'a str),
}