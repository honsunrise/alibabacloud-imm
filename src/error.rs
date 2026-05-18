use http::StatusCode;
use http::header::InvalidHeaderValue;
use thiserror::Error;
use url::ParseError;

use crate::response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] ParseError),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Invalid Header Value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Invalid Json: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("API error({status_code}): {message:?}")]
    ApiError {
        status_code: StatusCode,
        message: Option<Box<response::ErrorResponse>>,
    },

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
