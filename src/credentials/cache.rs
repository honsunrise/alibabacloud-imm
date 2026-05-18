use std::time::Duration;

use tokio::sync::RwLock;

use crate::Result;
use crate::credentials::{Credentials, CredentialsProvider};

const DEFAULT_REFRESH_SKEW: Duration = Duration::from_secs(5 * 60);

#[derive(Debug)]
pub struct CachingCredentialsProvider<P> {
    inner: P,
    cache: RwLock<Option<Credentials>>,
    refresh_skew: Duration,
}

impl<P> CachingCredentialsProvider<P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            cache: RwLock::new(None),
            refresh_skew: DEFAULT_REFRESH_SKEW,
        }
    }

    pub fn with_refresh_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = skew;
        self
    }
}

impl<P> CredentialsProvider for CachingCredentialsProvider<P>
where
    P: CredentialsProvider,
{
    async fn get_credentials(&self) -> Result<Credentials> {
        {
            let cached = self.cache.read().await;
            if let Some(c) = cached.as_ref()
                && !c.is_expired_within(self.refresh_skew)
            {
                return Ok(c.clone());
            }
        }

        let mut cached = self.cache.write().await;
        if let Some(c) = cached.as_ref()
            && !c.is_expired_within(self.refresh_skew)
        {
            return Ok(c.clone());
        }

        let fresh = self.inner.get_credentials().await?;
        *cached = Some(fresh.clone());
        Ok(fresh)
    }
}
