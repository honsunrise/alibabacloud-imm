//! File Process operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileCompressionTaskQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(rename = "SourceManifestURI", skip_serializing_if = "Option::is_none")]
    pub source_manifest_uri: Option<String>,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileCompressionTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a file compression task for batch download.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfilecompressiontask>
pub struct CreateFileCompressionTask {
    pub query: CreateFileCompressionTaskQuery,
}

impl Ops for CreateFileCompressionTask {
    const ACTION: &'static str = "CreateFileCompressionTask";
    type Query = CreateFileCompressionTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateFileCompressionTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateFileCompressionTaskOps {
    /// Create a file compression task for batch download.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfilecompressiontask>
    fn create_file_compression_task(
        &self,
        query: CreateFileCompressionTaskQuery,
    ) -> impl Future<Output = Result<CreateFileCompressionTaskResponse>>;
}

impl CreateFileCompressionTaskOps for Client {
    async fn create_file_compression_task(
        &self,
        query: CreateFileCompressionTaskQuery,
    ) -> Result<CreateFileCompressionTaskResponse> {
        self.request(CreateFileCompressionTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileUncompressionTaskQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_files: Option<serde_json::Value>,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
    #[serde(rename = "TargetURI", skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFileUncompressionTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a file decompression task supporting ZIP, RAR, and 7z formats.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfileuncompressiontask>
pub struct CreateFileUncompressionTask {
    pub query: CreateFileUncompressionTaskQuery,
}

impl Ops for CreateFileUncompressionTask {
    const ACTION: &'static str = "CreateFileUncompressionTask";
    type Query = CreateFileUncompressionTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateFileUncompressionTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateFileUncompressionTaskOps {
    /// Create a file decompression task supporting ZIP, RAR, and 7z formats.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfileuncompressiontask>
    fn create_file_uncompression_task(
        &self,
        query: CreateFileUncompressionTaskQuery,
    ) -> impl Future<Output = Result<CreateFileUncompressionTaskResponse>>;
}

impl CreateFileUncompressionTaskOps for Client {
    async fn create_file_uncompression_task(
        &self,
        query: CreateFileUncompressionTaskQuery,
    ) -> Result<CreateFileUncompressionTaskResponse> {
        self.request(CreateFileUncompressionTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateArchiveFileInspectionTaskQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateArchiveFileInspectionTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a task to inspect archive file contents without decompression.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createarchivefileinspectiontask>
pub struct CreateArchiveFileInspectionTask {
    pub query: CreateArchiveFileInspectionTaskQuery,
}

impl Ops for CreateArchiveFileInspectionTask {
    const ACTION: &'static str = "CreateArchiveFileInspectionTask";
    type Query = CreateArchiveFileInspectionTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateArchiveFileInspectionTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateArchiveFileInspectionTaskOps {
    /// Create a task to inspect archive file contents without decompression.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createarchivefileinspectiontask>
    fn create_archive_file_inspection_task(
        &self,
        query: CreateArchiveFileInspectionTaskQuery,
    ) -> impl Future<Output = Result<CreateArchiveFileInspectionTaskResponse>>;
}

impl CreateArchiveFileInspectionTaskOps for Client {
    async fn create_archive_file_inspection_task(
        &self,
        query: CreateArchiveFileInspectionTaskQuery,
    ) -> Result<CreateArchiveFileInspectionTaskResponse> {
        self.request(CreateArchiveFileInspectionTask { query }).await
    }
}

pub trait FileProcessOperations:
    CreateFileCompressionTaskOps + CreateFileUncompressionTaskOps + CreateArchiveFileInspectionTaskOps
{
}
impl<T> FileProcessOperations for T where
    T: CreateFileCompressionTaskOps + CreateFileUncompressionTaskOps + CreateArchiveFileInspectionTaskOps
{
}
