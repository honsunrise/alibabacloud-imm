use std::fmt;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::{Error, Result};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub request_id: String,
    #[serde(default)]
    pub host_id: String,
    #[serde(default)]
    pub recommend: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code={}, message={}, request_id={}",
            self.code, self.message, self.request_id
        )
    }
}

impl std::error::Error for ErrorResponse {}

async fn process_response_error(resp: reqwest::Response) -> Result<Error> {
    let status = resp.status();
    let text = resp.text().await?;
    let error = if text.trim().is_empty() {
        None
    } else {
        Some(Box::new(serde_json::from_str::<ErrorResponse>(&text)?))
    };
    Ok(Error::ApiError {
        status_code: status,
        message: error,
    })
}

pub trait ResponseProcessor {
    type Output;

    fn from_response(response: reqwest::Response) -> impl Future<Output = Result<Self::Output>>;
}

pub struct EmptyResponseProcessor;

impl ResponseProcessor for EmptyResponseProcessor {
    type Output = ();

    async fn from_response(resp: reqwest::Response) -> Result<()> {
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(process_response_error(resp).await?)
        }
    }
}

pub struct BodyResponseProcessor<T>(PhantomData<T>);

impl<T> ResponseProcessor for BodyResponseProcessor<T>
where
    T: DeserializeOwned,
{
    type Output = T;

    async fn from_response(resp: reqwest::Response) -> Result<Self::Output> {
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await?;
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(process_response_error(resp).await?)
        }
    }
}
