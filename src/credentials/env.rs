use std::env;

use super::{Credentials, CredentialsProvider};
use crate::{Error, Result};

#[derive(Debug)]
pub struct EnvironmentCredentialsProvider;

impl Default for EnvironmentCredentialsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentCredentialsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl CredentialsProvider for EnvironmentCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        let access_key_id = env::var("ALIBABA_CLOUD_ACCESS_KEY_ID")
            .or_else(|_| env::var("ALICLOUD_ACCESS_KEY_ID"))
            .map_err(|_| Error::InvalidCredentials)?;

        let access_key_secret = env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET")
            .or_else(|_| env::var("ALICLOUD_ACCESS_KEY_SECRET"))
            .map_err(|_| Error::InvalidCredentials)?;

        let security_token = env::var("ALIBABA_CLOUD_SECURITY_TOKEN")
            .or_else(|_| env::var("ALICLOUD_SECURITY_TOKEN"))
            .ok();

        Ok(Credentials {
            access_key_id,
            access_key_secret,
            security_token,
            expiration: None,
        })
    }
}
