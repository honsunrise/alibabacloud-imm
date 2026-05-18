use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use jiff::Timestamp;
use serde::Deserialize;
use tracing::debug;

use crate::credentials::{CachingCredentialsProvider, Credentials, CredentialsProvider};
use crate::{Error, Result};

const DEFAULT_STS_ENDPOINT: &str = "https://sts.aliyuncs.com";
const STS_API_VERSION: &str = "2015-04-01";
const DEFAULT_SESSION_DURATION: u32 = 3600;
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_REFRESH_SKEW: Duration = Duration::from_secs(5 * 60);

fn default_role_session_name() -> String {
    let ts = Timestamp::now().as_second();
    format!("alibabacloud-imm-rrsa-session-{ts}")
}

fn normalize_sts_endpoint(endpoint: impl Into<String>) -> String {
    let endpoint = endpoint.into();
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint
    } else {
        format!("https://{endpoint}")
    }
}

#[derive(Debug, Clone)]
pub struct RrsaCredentialsProviderBuilder {
    role_arn: Option<String>,
    oidc_provider_arn: Option<String>,
    oidc_token_file_path: Option<PathBuf>,
    role_session_name: Option<String>,
    policy: Option<String>,
    session_duration_seconds: u32,
    sts_endpoint: String,
    http_client: Option<reqwest::Client>,
    refresh_skew: Duration,
}

impl Default for RrsaCredentialsProviderBuilder {
    fn default() -> Self {
        Self {
            role_arn: None,
            oidc_provider_arn: None,
            oidc_token_file_path: None,
            role_session_name: None,
            policy: None,
            session_duration_seconds: DEFAULT_SESSION_DURATION,
            sts_endpoint: DEFAULT_STS_ENDPOINT.to_string(),
            http_client: None,
            refresh_skew: DEFAULT_REFRESH_SKEW,
        }
    }
}

impl RrsaCredentialsProviderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role_arn(mut self, arn: impl Into<String>) -> Self {
        self.role_arn = Some(arn.into());
        self
    }

    pub fn oidc_provider_arn(mut self, arn: impl Into<String>) -> Self {
        self.oidc_provider_arn = Some(arn.into());
        self
    }

    pub fn oidc_token_file_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.oidc_token_file_path = Some(path.into());
        self
    }

    pub fn role_session_name(mut self, name: impl Into<String>) -> Self {
        self.role_session_name = Some(name.into());
        self
    }

    pub fn policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    pub fn session_duration_seconds(mut self, seconds: u32) -> Self {
        self.session_duration_seconds = seconds;
        self
    }

    pub fn sts_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.sts_endpoint = normalize_sts_endpoint(endpoint);
        self
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    pub fn refresh_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = skew;
        self
    }

    pub fn build(self) -> Result<RrsaCredentialsProvider> {
        let role_arn = self
            .role_arn
            .ok_or_else(|| Error::InvalidArgument("rrsa: role_arn is required".to_string()))?;
        let oidc_provider_arn = self
            .oidc_provider_arn
            .ok_or_else(|| Error::InvalidArgument("rrsa: oidc_provider_arn is required".to_string()))?;
        let oidc_token_file_path = self
            .oidc_token_file_path
            .ok_or_else(|| Error::InvalidArgument("rrsa: oidc_token_file_path is required".to_string()))?;

        let http_client = self.http_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
                .build()
                .expect("default reqwest client")
        });

        Ok(RrsaCredentialsProvider {
            inner: CachingCredentialsProvider::new(RrsaInner {
                role_arn,
                oidc_provider_arn,
                oidc_token_file_path,
                role_session_name: self.role_session_name.unwrap_or_else(default_role_session_name),
                policy: self.policy,
                session_duration_seconds: self.session_duration_seconds,
                sts_endpoint: self.sts_endpoint,
                http_client,
            })
            .with_refresh_skew(self.refresh_skew),
        })
    }
}

#[derive(Debug)]
pub struct RrsaCredentialsProvider {
    inner: CachingCredentialsProvider<RrsaInner>,
}

impl RrsaCredentialsProvider {
    pub fn builder() -> RrsaCredentialsProviderBuilder {
        RrsaCredentialsProviderBuilder::new()
    }

    pub fn from_env(http_client: reqwest::Client) -> Option<Self> {
        let role_arn = env::var("ALIBABA_CLOUD_ROLE_ARN")
            .ok()
            .filter(|s| !s.is_empty())?;
        let oidc_provider_arn = env::var("ALIBABA_CLOUD_OIDC_PROVIDER_ARN")
            .ok()
            .filter(|s| !s.is_empty())?;
        let oidc_token_file = env::var("ALIBABA_CLOUD_OIDC_TOKEN_FILE")
            .ok()
            .filter(|s| !s.is_empty())?;

        let mut builder = Self::builder()
            .role_arn(role_arn)
            .oidc_provider_arn(oidc_provider_arn)
            .oidc_token_file_path(oidc_token_file)
            .http_client(http_client);

        if let Ok(name) = env::var("ALIBABA_CLOUD_ROLE_SESSION_NAME")
            && !name.is_empty()
        {
            builder = builder.role_session_name(name);
        }

        if let Ok(endpoint) = env::var("ALIBABA_CLOUD_STS_ENDPOINT")
            && !endpoint.is_empty()
        {
            builder = builder.sts_endpoint(normalize_sts_endpoint(endpoint));
        }

        if let Ok(s) = env::var("ALIBABA_CLOUD_SESSION_DURATION_SECONDS")
            && let Ok(secs) = s.parse::<u32>()
        {
            builder = builder.session_duration_seconds(secs);
        }

        if let Ok(s) = env::var("ALIBABA_CLOUD_CREDENTIALS_REFRESH_SKEW_SECONDS")
            && let Ok(secs) = s.parse::<u64>()
        {
            builder = builder.refresh_skew(Duration::from_secs(secs));
        }

        builder.build().ok()
    }
}

impl CredentialsProvider for RrsaCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials().await
    }
}

#[derive(Debug)]
struct RrsaInner {
    role_arn: String,
    oidc_provider_arn: String,
    oidc_token_file_path: PathBuf,
    role_session_name: String,
    policy: Option<String>,
    session_duration_seconds: u32,
    sts_endpoint: String,
    http_client: reqwest::Client,
}

impl CredentialsProvider for RrsaInner {
    async fn get_credentials(&self) -> Result<Credentials> {
        let token = read_token_file(&self.oidc_token_file_path).await?;
        assume_role_with_oidc(self, &token).await
    }
}

async fn read_token_file(path: &Path) -> Result<String> {
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|e| Error::Other(format!("rrsa: failed to read OIDC token file {}: {e}", path.display())))?;
    let token = String::from_utf8(bytes)
        .map_err(|_| Error::Other("rrsa: OIDC token file is not valid UTF-8".to_string()))?
        .trim()
        .to_string();
    if token.is_empty() {
        return Err(Error::Other("rrsa: OIDC token file is empty".to_string()));
    }
    Ok(token)
}

async fn assume_role_with_oidc(inner: &RrsaInner, oidc_token: &str) -> Result<Credentials> {
    let body = {
        let now = Timestamp::now();
        let timestamp = now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();

        let mut form = url::form_urlencoded::Serializer::new(String::new());
        form.append_pair("Action", "AssumeRoleWithOIDC");
        form.append_pair("Version", STS_API_VERSION);
        form.append_pair("Format", "JSON");
        form.append_pair("Timestamp", &timestamp);
        form.append_pair("RoleArn", &inner.role_arn);
        form.append_pair("OIDCProviderArn", &inner.oidc_provider_arn);
        form.append_pair("OIDCToken", oidc_token);
        form.append_pair("RoleSessionName", &inner.role_session_name);
        form.append_pair("DurationSeconds", &inner.session_duration_seconds.to_string());
        if let Some(policy) = &inner.policy {
            form.append_pair("Policy", policy);
        }
        form.finish()
    };

    debug!(
        target: "alibabacloud_imm::credentials::rrsa",
        role_arn = %inner.role_arn,
        oidc_provider_arn = %inner.oidc_provider_arn,
        "calling AssumeRoleWithOIDC",
    );

    let response = inner
        .http_client
        .post(&inner.sts_endpoint)
        .header(http::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(http::header::ACCEPT, "application/json")
        .body(body)
        .send()
        .await?;

    let status = response.status();
    let bytes = response.bytes().await?;
    let text = String::from_utf8_lossy(&bytes);
    if !status.is_success() {
        return Err(Error::Other(format!(
            "rrsa: AssumeRoleWithOIDC failed with status {status}: {text}"
        )));
    }

    let parsed: AssumeRoleWithOidcResponse = serde_json::from_slice(&bytes).map_err(|e| {
        Error::Other(format!("rrsa: failed to parse AssumeRoleWithOIDC response: {e}, body: {text}"))
    })?;

    let creds = parsed.credentials.ok_or_else(|| {
        Error::Other(format!("rrsa: AssumeRoleWithOIDC response missing Credentials, body: {text}"))
    })?;

    let expiration = creds
        .expiration
        .parse::<Timestamp>()
        .map_err(|e| Error::Other(format!("rrsa: failed to parse expiration `{}`: {e}", creds.expiration)))?;

    Ok(Credentials::with_sts(
        creds.access_key_id,
        creds.access_key_secret,
        creds.security_token,
        Some(expiration),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AssumeRoleWithOidcResponse {
    #[serde(default)]
    credentials: Option<StsCredentials>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StsCredentials {
    access_key_id: String,
    access_key_secret: String,
    security_token: String,
    expiration: String,
}
