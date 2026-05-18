//! Oss Bucket operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttachOSSBucketQuery {
    pub project_name: String,
    #[serde(rename = "OSSBucket")]
    pub oss_bucket: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttachOSSBucketResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Bind an OSS bucket to an IMM project to use IMM data processing capabilities via OSS x-oss-process.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-attachossbucket>
pub struct AttachOSSBucket {
    pub query: AttachOSSBucketQuery,
}

impl Ops for AttachOSSBucket {
    const ACTION: &'static str = "AttachOSSBucket";
    type Query = AttachOSSBucketQuery;
    type Body = ();
    type Response = BodyResponseProcessor<AttachOSSBucketResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait AttachOSSBucketOps {
    /// Bind an OSS bucket to an IMM project to use IMM data processing capabilities via OSS x-oss-process.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-attachossbucket>
    fn attach_oss_bucket(
        &self,
        query: AttachOSSBucketQuery,
    ) -> impl Future<Output = Result<AttachOSSBucketResponse>>;
}

impl AttachOSSBucketOps for Client {
    async fn attach_oss_bucket(&self, query: AttachOSSBucketQuery) -> Result<AttachOSSBucketResponse> {
        self.request(AttachOSSBucket { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetachOSSBucketQuery {
    #[serde(rename = "OSSBucket")]
    pub oss_bucket: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetachOSSBucketResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Unbind an OSS bucket from an IMM project.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detachossbucket>
pub struct DetachOSSBucket {
    pub query: DetachOSSBucketQuery,
}

impl Ops for DetachOSSBucket {
    const ACTION: &'static str = "DetachOSSBucket";
    type Query = DetachOSSBucketQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetachOSSBucketResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetachOSSBucketOps {
    /// Unbind an OSS bucket from an IMM project.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detachossbucket>
    fn detach_oss_bucket(
        &self,
        query: DetachOSSBucketQuery,
    ) -> impl Future<Output = Result<DetachOSSBucketResponse>>;
}

impl DetachOSSBucketOps for Client {
    async fn detach_oss_bucket(&self, query: DetachOSSBucketQuery) -> Result<DetachOSSBucketResponse> {
        self.request(DetachOSSBucket { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOSSBucketAttachmentQuery {
    #[serde(rename = "OSSBucket")]
    pub oss_bucket: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOSSBucketAttachmentResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
}

/// Get the project name bound to a specified OSS bucket.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getossbucketattachment>
pub struct GetOSSBucketAttachment {
    pub query: GetOSSBucketAttachmentQuery,
}

impl Ops for GetOSSBucketAttachment {
    const ACTION: &'static str = "GetOSSBucketAttachment";
    type Query = GetOSSBucketAttachmentQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetOSSBucketAttachmentResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetOSSBucketAttachmentOps {
    /// Get the project name bound to a specified OSS bucket.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getossbucketattachment>
    fn get_oss_bucket_attachment(
        &self,
        query: GetOSSBucketAttachmentQuery,
    ) -> impl Future<Output = Result<GetOSSBucketAttachmentResponse>>;
}

impl GetOSSBucketAttachmentOps for Client {
    async fn get_oss_bucket_attachment(
        &self,
        query: GetOSSBucketAttachmentQuery,
    ) -> Result<GetOSSBucketAttachmentResponse> {
        self.request(GetOSSBucketAttachment { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedOSSBucketsQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAttachedOSSBucketsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(rename = "AttachedOSSBuckets", default)]
    pub attached_oss_buckets: Vec<serde_json::Value>,
}

/// List all OSS buckets bound to a project.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listattachedossbuckets>
pub struct ListAttachedOSSBuckets {
    pub query: ListAttachedOSSBucketsQuery,
}

impl Ops for ListAttachedOSSBuckets {
    const ACTION: &'static str = "ListAttachedOSSBuckets";
    type Query = ListAttachedOSSBucketsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListAttachedOSSBucketsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListAttachedOSSBucketsOps {
    /// List all OSS buckets bound to a project.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listattachedossbuckets>
    fn list_attached_oss_buckets(
        &self,
        query: ListAttachedOSSBucketsQuery,
    ) -> impl Future<Output = Result<ListAttachedOSSBucketsResponse>>;
}

impl ListAttachedOSSBucketsOps for Client {
    async fn list_attached_oss_buckets(
        &self,
        query: ListAttachedOSSBucketsQuery,
    ) -> Result<ListAttachedOSSBucketsResponse> {
        self.request(ListAttachedOSSBuckets { query }).await
    }
}

pub trait OssBucketOperations:
    AttachOSSBucketOps + DetachOSSBucketOps + GetOSSBucketAttachmentOps + ListAttachedOSSBucketsOps
{
}
impl<T> OssBucketOperations for T where
    T: AttachOSSBucketOps + DetachOSSBucketOps + GetOSSBucketAttachmentOps + ListAttachedOSSBucketsOps
{
}
