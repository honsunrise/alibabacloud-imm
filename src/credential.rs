use std::collections::BTreeMap;

use hmac::{Hmac, KeyInit, Mac};
use http::HeaderMap;
use jiff::Timestamp;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::Result;
use crate::credentials::Credentials;

type HmacSha256 = Hmac<Sha256>;

pub(crate) struct SignContext {
    pub sorted_query: BTreeMap<String, String>,
}

pub(crate) fn sign_request(
    credentials: &Credentials,
    request: &mut reqwest::Request,
    ctx: SignContext,
) -> Result<()> {
    let now = Timestamp::now();
    let date = now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();
    let nonce = Uuid::new_v4().to_string();

    let body_bytes = request.body().and_then(|b| b.as_bytes()).unwrap_or(&[]);
    let body_hash = hex::encode(Sha256::digest(body_bytes));

    let headers = request.headers_mut();
    headers.insert("x-acs-date", http::HeaderValue::from_str(&date)?);
    headers.insert("x-acs-signature-nonce", http::HeaderValue::from_str(&nonce)?);
    headers.insert("x-acs-content-sha256", http::HeaderValue::from_str(&body_hash)?);

    let signed_headers = build_signed_headers(headers);
    let method = request.method().as_str().to_owned();
    let path = request.url().path().to_owned();
    let canonical_request = build_canonical_request(
        &method,
        &path,
        &ctx.sorted_query,
        request.headers(),
        &signed_headers,
        &body_hash,
    );

    let hashed_request = hex::encode(Sha256::digest(canonical_request.as_bytes()));
    let string_to_sign = format!("ACS3-HMAC-SHA256\n{hashed_request}");

    let mut mac = HmacSha256::new_from_slice(credentials.access_key_secret.as_bytes()).unwrap();
    mac.update(string_to_sign.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    let authorization = format!(
        "ACS3-HMAC-SHA256 Credential={},SignedHeaders={},Signature={}",
        credentials.access_key_id, signed_headers, signature
    );

    let headers = request.headers_mut();
    headers.insert(http::header::AUTHORIZATION, http::HeaderValue::from_str(&authorization)?);

    Ok(())
}

fn build_signed_headers(headers: &HeaderMap) -> String {
    let mut header_names: Vec<&str> = headers
        .keys()
        .map(|k| k.as_str())
        .filter(|k| *k == "host" || *k == "content-type" || k.starts_with("x-acs-"))
        .collect();
    header_names.sort_unstable();
    header_names.dedup();
    header_names.join(";")
}

fn build_canonical_request(
    method: &str,
    path: &str,
    sorted_query: &BTreeMap<String, String>,
    headers: &HeaderMap,
    signed_headers: &str,
    body_hash: &str,
) -> String {
    let canonical_query = sorted_query
        .iter()
        .map(|(k, v)| format!("{}={}", acs_percent_encode(k), acs_percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let canonical_headers = build_canonical_headers(headers, signed_headers);

    format!("{method}\n{path}\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{body_hash}")
}

fn build_canonical_headers(headers: &HeaderMap, signed_headers: &str) -> String {
    let signed: Vec<&str> = signed_headers.split(';').collect();
    let mut result = String::new();
    for name in &signed {
        if let Some(value) = headers.get(*name) {
            if let Ok(v) = value.to_str() {
                result.push_str(name);
                result.push(':');
                result.push_str(v.trim());
                result.push('\n');
            }
        }
    }
    result
}

pub(crate) fn acs_percent_encode(s: &str) -> String {
    use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
    const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    utf8_percent_encode(s, ENCODE_SET).to_string()
}
