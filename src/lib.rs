mod credential;
pub mod credentials;
mod error;
pub mod ops;
pub mod region;
pub mod response;

use std::collections::BTreeMap;
use std::time::Duration;

use serde::Serialize;
use tracing::trace;
use url::Url;

use self::credential::SignContext;
use self::credentials::{
    CredentialsProvider,
    DefaultCredentialsChain,
    DynCredentialsProvider,
    StaticCredentialsProvider,
};
pub use self::error::{Error, Result};
pub use self::region::Region;
pub use self::response::ResponseProcessor;

pub trait Ops: Sized {
    const ACTION: &'static str;
    const VERSION: &'static str = "2020-09-30";

    type Query: Serialize;
    type Body: Serialize;
    type Response: ResponseProcessor;

    fn into_parts(self) -> (Self::Query, Self::Body);
}

pub(crate) trait Request<P> {
    type Response;

    fn request(&self, ops: P) -> impl Future<Output = Result<Self::Response>>;
}

pub struct ClientConfig {
    pub http_timeout: Duration,
    pub default_headers: http::HeaderMap,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    endpoint: String,
    credentials_provider: DynCredentialsProvider,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    async fn prepare_request<P>(&self, ops: P) -> Result<reqwest::Request>
    where
        P: Ops + Send + 'static,
        P::Query: Serialize + Send,
        P::Body: Serialize + Send,
        P::Response: ResponseProcessor + Send,
    {
        let (query, body) = ops.into_parts();

        let mut url = Url::parse(&self.endpoint)?;
        url.set_path("/");

        let query_pairs = serialize_to_pairs(&query);
        for (k, v) in &query_pairs {
            url.query_pairs_mut().append_pair(k, v);
        }

        let body_pairs = serialize_to_pairs(&body);
        let form_body = if body_pairs.is_empty() {
            String::new()
        } else {
            let mut buf = String::new();
            for (i, (k, v)) in body_pairs.iter().enumerate() {
                if i > 0 {
                    buf.push('&');
                }
                buf.push_str(&credential::acs_percent_encode(k));
                buf.push('=');
                buf.push_str(&credential::acs_percent_encode(v));
            }
            buf
        };

        let mut request = self
            .http_client
            .request(http::Method::POST, url.clone())
            .build()?;

        let headers = request.headers_mut();
        headers.insert(http::header::HOST, host_header_value(&url)?);
        headers.insert("x-acs-action", http::HeaderValue::from_static(P::ACTION));
        headers.insert("x-acs-version", http::HeaderValue::from_static(P::VERSION));

        if !form_body.is_empty() {
            headers.insert(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            let len = form_body.len().to_string();
            headers.insert(http::header::CONTENT_LENGTH, http::HeaderValue::from_str(&len)?);
            *request.body_mut() = Some(reqwest::Body::from(form_body));
        }

        let credentials = self.credentials_provider.get_credentials().await?;

        if let Some(ref token) = credentials.security_token {
            request
                .headers_mut()
                .insert("x-acs-security-token", http::HeaderValue::from_str(token)?);
        }

        let sorted_query: BTreeMap<String, String> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        let sign_context = SignContext { sorted_query };

        credential::sign_request(&credentials, &mut request, sign_context)?;

        Ok(request)
    }
}

fn host_header_value(url: &Url) -> Result<http::HeaderValue> {
    let mut host = url.host().map(|host| host.to_string()).unwrap_or_default();
    if let Some(port) = url.port() {
        host.push(':');
        host.push_str(&port.to_string());
    }
    Ok(http::HeaderValue::from_str(&host)?)
}

fn serialize_to_pairs<T: Serialize>(value: &T) -> Vec<(String, String)> {
    let json_val = match serde_json::to_value(value) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let map = match json_val {
        serde_json::Value::Object(m) => m,
        _ => return Vec::new(),
    };

    let mut pairs = Vec::new();
    for (k, v) in map {
        match v {
            serde_json::Value::Null => {},
            serde_json::Value::String(s) => {
                if !s.is_empty() {
                    pairs.push((k, s));
                }
            },
            serde_json::Value::Bool(b) => {
                pairs.push((k, b.to_string()));
            },
            serde_json::Value::Number(n) => {
                pairs.push((k, n.to_string()));
            },
            serde_json::Value::Array(ref a) if a.is_empty() => {},
            serde_json::Value::Object(ref o) if o.is_empty() => {},
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                if let Ok(s) = serde_json::to_string(&v) {
                    pairs.push((k, s));
                }
            },
        }
    }
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

impl<P> Request<P> for Client
where
    P: Ops + Send + 'static,
    P::Query: Serialize + Send,
    P::Body: Serialize + Send,
    P::Response: ResponseProcessor + Send,
{
    type Response = <P::Response as ResponseProcessor>::Output;

    async fn request(&self, ops: P) -> Result<Self::Response> {
        let request = self.prepare_request(ops).await?;

        trace!("Sending request: {request:?}");
        let resp = self.http_client.execute(request).await?;

        P::Response::from_response(resp).await
    }
}

pub struct ClientBuilder {
    config: ClientConfig,
    endpoint: Option<String>,
    region: Option<Region>,
    vpc: bool,
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    security_token: Option<String>,
    credentials_provider: Option<DynCredentialsProvider>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
            endpoint: None,
            region: None,
            vpc: false,
            access_key_id: None,
            access_key_secret: None,
            security_token: None,
            credentials_provider: None,
        }
    }

    pub fn endpoint<T: AsRef<str>>(mut self, endpoint: T) -> Self {
        self.endpoint = Some(endpoint.as_ref().to_string());
        self
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Use the VPC internal endpoint instead of the public endpoint.
    ///
    /// When set, the endpoint is derived as `https://imm-vpc.{region}.aliyuncs.com`
    /// instead of `https://imm.{region}.aliyuncs.com`. Has no effect if an
    /// explicit [`endpoint`](Self::endpoint) is provided.
    pub fn vpc(mut self) -> Self {
        self.vpc = true;
        self
    }

    pub fn access_key_id<T: AsRef<str>>(mut self, access_key_id: T) -> Self {
        self.access_key_id = Some(access_key_id.as_ref().to_string());
        self
    }

    pub fn access_key_secret<T: AsRef<str>>(mut self, access_key_secret: T) -> Self {
        self.access_key_secret = Some(access_key_secret.as_ref().to_string());
        self
    }

    pub fn security_token<T: AsRef<str>>(mut self, security_token: T) -> Self {
        self.security_token = Some(security_token.as_ref().to_string());
        self
    }

    pub fn credentials_provider<P>(mut self, provider: P) -> Self
    where
        P: CredentialsProvider + 'static,
    {
        self.credentials_provider = Some(DynCredentialsProvider::new(provider));
        self
    }

    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.config.http_timeout = timeout;
        self
    }

    pub fn default_headers(mut self, headers: http::HeaderMap) -> Self {
        self.config.default_headers = headers;
        self
    }

    pub fn build(self) -> Result<Client> {
        let endpoint = if let Some(ep) = self.endpoint {
            ep
        } else {
            let region = self
                .region
                .as_ref()
                .ok_or_else(|| Error::InvalidArgument("either endpoint or region is required".to_string()))?;
            if self.vpc {
                region.vpc_endpoint()
            } else {
                region.public_endpoint()
            }
        };

        let http_client = reqwest::Client::builder()
            .default_headers(self.config.default_headers)
            .timeout(self.config.http_timeout)
            .build()?;

        let credentials_provider = if let Some(provider) = self.credentials_provider {
            provider
        } else {
            match (self.access_key_id, self.access_key_secret) {
                (Some(ak), Some(sk)) => {
                    let provider = if let Some(token) = self.security_token {
                        StaticCredentialsProvider::with_security_token(ak, sk, token)
                    } else {
                        StaticCredentialsProvider::new(ak, sk)
                    };
                    DynCredentialsProvider::new(provider)
                },
                _ => DynCredentialsProvider::new(DefaultCredentialsChain::new()),
            }
        };

        Ok(Client {
            http_client,
            endpoint,
            credentials_provider,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::EmptyResponseProcessor;

    #[derive(Serialize)]
    struct Empty;

    struct TestOps;

    impl Ops for TestOps {
        const ACTION: &'static str = "TestAction";

        type Body = Empty;
        type Query = Empty;
        type Response = EmptyResponseProcessor;

        fn into_parts(self) -> (Self::Query, Self::Body) {
            (Empty, Empty)
        }
    }

    #[cfg(feature = "default-tls")]
    #[test]
    fn default_region_client_builds_with_default_tls() {
        Client::builder().region(Region::CnShanghai).build().unwrap();
    }

    #[test]
    fn host_header_value_preserves_ipv6_endpoint_port() {
        let url = Url::parse("http://[::1]:9000").unwrap();

        assert_eq!(host_header_value(&url).unwrap().to_str().unwrap(), "[::1]:9000");
    }

    #[tokio::test]
    async fn prepare_request_preserves_custom_endpoint_port_in_host_header() {
        let client = Client::builder()
            .endpoint("http://127.0.0.1:9000")
            .access_key_id("test-ak")
            .access_key_secret("test-sk")
            .build()
            .unwrap();

        let request = client.prepare_request(TestOps).await.unwrap();

        assert_eq!(
            request
                .headers()
                .get(http::header::HOST)
                .unwrap()
                .to_str()
                .unwrap(),
            "127.0.0.1:9000"
        );
    }
}
