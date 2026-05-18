use tracing::debug;

use crate::Result;
use crate::credentials::env::EnvironmentCredentialsProvider;
use crate::credentials::rrsa::RrsaCredentialsProvider;
use crate::credentials::{Credentials, CredentialsProvider, DynCredentialsProvider};

pub struct CredentialsChain {
    providers: Vec<(String, DynCredentialsProvider)>,
}

impl std::fmt::Debug for CredentialsChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<_> = self.providers.iter().map(|(n, _)| n.as_str()).collect();
        f.debug_struct("CredentialsChain")
            .field("providers", &names)
            .finish()
    }
}

impl CredentialsChain {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn push<P>(mut self, name: impl Into<String>, provider: P) -> Self
    where
        P: CredentialsProvider + 'static,
    {
        self.providers
            .push((name.into(), DynCredentialsProvider::new(provider)));
        self
    }
}

impl Default for CredentialsChain {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsProvider for CredentialsChain {
    async fn get_credentials(&self) -> Result<Credentials> {
        let mut last_error = None;
        for (name, provider) in &self.providers {
            match provider.get_credentials().await {
                Ok(c) => return Ok(c),
                Err(e) => {
                    debug!(target: "alibabacloud_imm::credentials", "provider `{name}` failed: {e}");
                    last_error = Some(e);
                },
            }
        }
        Err(last_error.unwrap_or(crate::Error::InvalidCredentials))
    }
}

#[derive(Debug)]
pub struct DefaultCredentialsChain {
    inner: CredentialsChain,
}

#[derive(Default)]
pub struct DefaultCredentialsChainBuilder {
    http_client: Option<reqwest::Client>,
}

impl DefaultCredentialsChainBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    pub fn build(self) -> DefaultCredentialsChain {
        let http_client = self.http_client.unwrap_or_default();
        let mut chain = CredentialsChain::new().push("environment", EnvironmentCredentialsProvider);

        if let Some(rrsa) = RrsaCredentialsProvider::from_env(http_client) {
            chain = chain.push("rrsa", rrsa);
        }

        DefaultCredentialsChain { inner: chain }
    }
}

impl DefaultCredentialsChain {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn with_http_client(http_client: reqwest::Client) -> Self {
        Self::builder().http_client(http_client).build()
    }

    pub fn builder() -> DefaultCredentialsChainBuilder {
        DefaultCredentialsChainBuilder::new()
    }
}

impl Default for DefaultCredentialsChain {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsProvider for DefaultCredentialsChain {
    async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials().await
    }
}
