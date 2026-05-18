mod cache;
mod chain;
mod env;
pub mod rrsa;
mod r#static;

use std::sync::Arc;

use jiff::Timestamp;

pub use self::cache::CachingCredentialsProvider;
pub use self::chain::{CredentialsChain, DefaultCredentialsChain, DefaultCredentialsChainBuilder};
pub use self::env::EnvironmentCredentialsProvider;
pub use self::rrsa::{RrsaCredentialsProvider, RrsaCredentialsProviderBuilder};
pub use self::r#static::StaticCredentialsProvider;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Credentials {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: Option<String>,
    pub expiration: Option<Timestamp>,
}

impl Credentials {
    pub fn new(access_key_id: impl Into<String>, access_key_secret: impl Into<String>) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: None,
            expiration: None,
        }
    }

    pub fn with_sts(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
        expiration: Option<Timestamp>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: Some(security_token.into()),
            expiration,
        }
    }

    pub fn is_expired_within(&self, skew: std::time::Duration) -> bool {
        match self.expiration {
            None => false,
            Some(exp) => {
                let now = Timestamp::now();
                let skew = jiff::Span::try_from(skew).unwrap_or_else(|_| jiff::Span::new());
                let deadline = match now.checked_add(skew) {
                    Ok(d) => d,
                    Err(_) => return true,
                };
                deadline >= exp
            },
        }
    }
}

pub trait CredentialsProvider: Send + Sync + std::fmt::Debug {
    fn get_credentials(&self) -> impl Future<Output = Result<Credentials>> + Send;
}

#[derive(Clone)]
pub struct DynCredentialsProvider {
    inner: Arc<dyn DynCredentialsProviderImpl>,
}

impl std::fmt::Debug for DynCredentialsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt_debug(f)
    }
}

impl DynCredentialsProvider {
    pub fn new<P>(provider: P) -> Self
    where
        P: CredentialsProvider + 'static,
    {
        Self {
            inner: Arc::new(provider),
        }
    }

    pub(crate) async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials_dyn().await
    }
}

trait DynCredentialsProviderImpl: Send + Sync + 'static {
    fn get_credentials_dyn(&self)
    -> std::pin::Pin<Box<dyn Future<Output = Result<Credentials>> + Send + '_>>;
    fn fmt_debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<P> DynCredentialsProviderImpl for P
where
    P: CredentialsProvider + 'static,
{
    fn get_credentials_dyn(
        &self,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<Credentials>> + Send + '_>> {
        Box::pin(self.get_credentials())
    }
    fn fmt_debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
