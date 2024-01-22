use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScrapeError {
    #[error("A CSS Selector in the '{src}' scraper failed. Message: {err}")]
    CSSSelectorFailed {
        src: String,
        err: String,
    },
    #[error("The HTML structure assumed in the '{src}' scraper failed.")]
    InvalidStructureAssumed {
        src: String,
    },
    #[error("Failed to parse a scraped string value in the '{src}' scraper. Message: {err}")]
    FailedToParseStringValue {
        src: String,
        err: String,
    },
    #[error("Failed to connect to url: {url}. Message: {err}")]
    FailedToConnect {
        url: String,
        err: String,
    },
    #[error("Failed to parse HTML from url: {url}. Message: {err}")]
    FailedToParseHtml {
        url: String,
        err: String,
    },
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Failed to connect to DB. Message: {err}")]
    FailedToConnect {
        err: String
    }
}