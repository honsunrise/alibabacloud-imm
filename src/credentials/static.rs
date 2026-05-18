use super::{Credentials, CredentialsProvider};
use crate::Result;

#[derive(Debug)]
pub struct StaticCredentialsProvider {
    credentials: Credentials,
}

impl StaticCredentialsProvider {
    pub fn new(access_key_id: impl Into<String>, access_key_secret: impl Into<String>) -> Self {
        Self {
            credentials: Credentials::new(access_key_id, access_key_secret),
        }
    }

    pub fn with_security_token(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
    ) -> Self {
        Self {
            credentials: Credentials::with_sts(access_key_id, access_key_secret, security_token, None),
        }
    }
}

impl CredentialsProvider for StaticCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        Ok(self.credentials.clone())
    }
}

impl From<Credentials> for StaticCredentialsProvider {
    fn from(credentials: Credentials) -> Self {
        Self { credentials }
    }
}
