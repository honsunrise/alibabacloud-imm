//! File Meta operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Index file metadata with data processing such as label detection, face detection, and location detection.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-indexfilemeta>
pub struct IndexFileMeta {
    pub query: IndexFileMetaQuery,
}

impl Ops for IndexFileMeta {
    const ACTION: &'static str = "IndexFileMeta";
    type Query = IndexFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<IndexFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait IndexFileMetaOps {
    /// Index file metadata with data processing such as label detection, face detection, and location detection.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-indexfilemeta>
    fn index_file_meta(
        &self,
        query: IndexFileMetaQuery,
    ) -> impl Future<Output = Result<IndexFileMetaResponse>>;
}

impl IndexFileMetaOps for Client {
    async fn index_file_meta(&self, query: IndexFileMetaQuery) -> Result<IndexFileMetaResponse> {
        self.request(IndexFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
}

/// Get metadata of an indexed file in a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getfilemeta>
pub struct GetFileMeta {
    pub query: GetFileMetaQuery,
}

impl Ops for GetFileMeta {
    const ACTION: &'static str = "GetFileMeta";
    type Query = GetFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetFileMetaOps {
    /// Get metadata of an indexed file in a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getfilemeta>
    fn get_file_meta(&self, query: GetFileMetaQuery) -> impl Future<Output = Result<GetFileMetaResponse>>;
}

impl GetFileMetaOps for Client {
    async fn get_file_meta(&self, query: GetFileMetaQuery) -> Result<GetFileMetaResponse> {
        self.request(GetFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub file: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update metadata of an indexed file in a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatefilemeta>
pub struct UpdateFileMeta {
    pub query: UpdateFileMetaQuery,
}

impl Ops for UpdateFileMeta {
    const ACTION: &'static str = "UpdateFileMeta";
    type Query = UpdateFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<UpdateFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait UpdateFileMetaOps {
    /// Update metadata of an indexed file in a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatefilemeta>
    fn update_file_meta(
        &self,
        query: UpdateFileMetaQuery,
    ) -> impl Future<Output = Result<UpdateFileMetaResponse>>;
}

impl UpdateFileMetaOps for Client {
    async fn update_file_meta(&self, query: UpdateFileMetaQuery) -> Result<UpdateFileMetaResponse> {
        self.request(UpdateFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "URI")]
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete file metadata from a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletefilemeta>
pub struct DeleteFileMeta {
    pub query: DeleteFileMetaQuery,
}

impl Ops for DeleteFileMeta {
    const ACTION: &'static str = "DeleteFileMeta";
    type Query = DeleteFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DeleteFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DeleteFileMetaOps {
    /// Delete file metadata from a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletefilemeta>
    fn delete_file_meta(
        &self,
        query: DeleteFileMetaQuery,
    ) -> impl Future<Output = Result<DeleteFileMetaResponse>>;
}

impl DeleteFileMetaOps for Client {
    async fn delete_file_meta(&self, query: DeleteFileMetaQuery) -> Result<DeleteFileMetaResponse> {
        self.request(DeleteFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchIndexFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub files: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchIndexFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Index file metadata in batch with data processing such as label detection, face detection, etc.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchindexfilemeta>
pub struct BatchIndexFileMeta {
    pub query: BatchIndexFileMetaQuery,
}

impl Ops for BatchIndexFileMeta {
    const ACTION: &'static str = "BatchIndexFileMeta";
    type Query = BatchIndexFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<BatchIndexFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait BatchIndexFileMetaOps {
    /// Index file metadata in batch with data processing such as label detection, face detection, etc.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchindexfilemeta>
    fn batch_index_file_meta(
        &self,
        query: BatchIndexFileMetaQuery,
    ) -> impl Future<Output = Result<BatchIndexFileMetaResponse>>;
}

impl BatchIndexFileMetaOps for Client {
    async fn batch_index_file_meta(
        &self,
        query: BatchIndexFileMetaQuery,
    ) -> Result<BatchIndexFileMetaResponse> {
        self.request(BatchIndexFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchGetFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "URIs")]
    pub ur_is: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchGetFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
}

/// Get metadata of indexed files from a dataset in batch.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchgetfilemeta>
pub struct BatchGetFileMeta {
    pub query: BatchGetFileMetaQuery,
}

impl Ops for BatchGetFileMeta {
    const ACTION: &'static str = "BatchGetFileMeta";
    type Query = BatchGetFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<BatchGetFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait BatchGetFileMetaOps {
    /// Get metadata of indexed files from a dataset in batch.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchgetfilemeta>
    fn batch_get_file_meta(
        &self,
        query: BatchGetFileMetaQuery,
    ) -> impl Future<Output = Result<BatchGetFileMetaResponse>>;
}

impl BatchGetFileMetaOps for Client {
    async fn batch_get_file_meta(&self, query: BatchGetFileMetaQuery) -> Result<BatchGetFileMetaResponse> {
        self.request(BatchGetFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchUpdateFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub files: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchUpdateFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
}

/// Update metadata of indexed files in a dataset in batch.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchupdatefilemeta>
pub struct BatchUpdateFileMeta {
    pub query: BatchUpdateFileMetaQuery,
}

impl Ops for BatchUpdateFileMeta {
    const ACTION: &'static str = "BatchUpdateFileMeta";
    type Query = BatchUpdateFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<BatchUpdateFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait BatchUpdateFileMetaOps {
    /// Update metadata of indexed files in a dataset in batch.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchupdatefilemeta>
    fn batch_update_file_meta(
        &self,
        query: BatchUpdateFileMetaQuery,
    ) -> impl Future<Output = Result<BatchUpdateFileMetaResponse>>;
}

impl BatchUpdateFileMetaOps for Client {
    async fn batch_update_file_meta(
        &self,
        query: BatchUpdateFileMetaQuery,
    ) -> Result<BatchUpdateFileMetaResponse> {
        self.request(BatchUpdateFileMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchDeleteFileMetaQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "URIs")]
    pub ur_is: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchDeleteFileMetaResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete file metadata from a dataset in batch.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchdeletefilemeta>
pub struct BatchDeleteFileMeta {
    pub query: BatchDeleteFileMetaQuery,
}

impl Ops for BatchDeleteFileMeta {
    const ACTION: &'static str = "BatchDeleteFileMeta";
    type Query = BatchDeleteFileMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<BatchDeleteFileMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait BatchDeleteFileMetaOps {
    /// Delete file metadata from a dataset in batch.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchdeletefilemeta>
    fn batch_delete_file_meta(
        &self,
        query: BatchDeleteFileMetaQuery,
    ) -> impl Future<Output = Result<BatchDeleteFileMetaResponse>>;
}

impl BatchDeleteFileMetaOps for Client {
    async fn batch_delete_file_meta(
        &self,
        query: BatchDeleteFileMetaQuery,
    ) -> Result<BatchDeleteFileMetaResponse> {
        self.request(BatchDeleteFileMeta { query }).await
    }
}

pub trait FileMetaOperations:
    IndexFileMetaOps
    + GetFileMetaOps
    + UpdateFileMetaOps
    + DeleteFileMetaOps
    + BatchIndexFileMetaOps
    + BatchGetFileMetaOps
    + BatchUpdateFileMetaOps
    + BatchDeleteFileMetaOps
{
}
impl<T> FileMetaOperations for T where
    T: IndexFileMetaOps
        + GetFileMetaOps
        + UpdateFileMetaOps
        + DeleteFileMetaOps
        + BatchIndexFileMetaOps
        + BatchGetFileMetaOps
        + BatchUpdateFileMetaOps
        + BatchDeleteFileMetaOps
{
}
