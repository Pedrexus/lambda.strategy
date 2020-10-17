use crate::sources::models::Source;
use crate::sources::yahoo::period::CandlestickInterval;
use snafu::Snafu;

/// All possible errors that can occur when using yahoo finance
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum InnerError {
    #[snafu(display("{:?} returned invalid data - {}", api, source.to_string()))]
    BadData {
        api: Source,
        source: serde_json::Error,
    },

    #[snafu(display(
        "{:?} call failed. '{}' returned a {} result.",
        api,
        url,
        status
    ))]
    CallFailed {
        api: Source,
        url: String,
        status: u16,
    },

    #[snafu(display(
        "{:?} chart failed to load {} - {}.",
        api,
        code,
        description
    ))]
    ChartFailed {
        api: Source,
        code: String,
        description: String,
    },

    #[snafu(display(
        "An internal error occurred - please report that '{}'",
        reason
    ))]
    InternalLogic { reason: String },

    #[snafu(display("An internal error occurred - please report that '{}' cannot be parsed because {}", url, source.to_string()))]
    InternalURL {
        url: String,
        source: url::ParseError,
    },

    #[snafu(display("API of name {} is not registered.", name))]
    InvalidApiName { name: String },

    #[snafu(display("Start date cannot be after the end date"))]
    InvalidStartDate,

    #[snafu(display("{:?} returned invalid data - {}", api, reason))]
    MissingData { api: Source, reason: String },

    #[snafu(display("Intraday intervals like {} are not allowed", interval))]
    NoIntraday { interval: CandlestickInterval },

    #[snafu(display("{:?} call failed for unknown reason.", api))]
    RequestFailed { api: Source, source: reqwest::Error },

    #[snafu(display(
        "Unexpected {:?} failure. '{}' returned a {}",
        api,
        url,
        code
    ))]
    UnexpectedFailure { api: Source, url: String, code: u16 },

    #[snafu(display("Unexpected error while reading data from '{}'", url))]
    UnexpectedErrorRead { url: String, source: reqwest::Error },

    #[snafu(display("{:?} call failed.  Expected data is missing.", api))]
    UnexpectedError { api: Source },

    #[snafu(display("Unexpected error from {:?} - data missing", api))]
    Unknown { api: Source },

    #[snafu(display(
        "We currently do not support securities of type '{}'",
        kind
    ))]
    UnsupportedSecurity { kind: String },
}

#[derive(Debug, Snafu)]
pub struct Error(InnerError);

pub type Result<T> = std::result::Result<T, Error>;
