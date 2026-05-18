//! Batch operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBatchBody {
    pub project_name: String,
    pub input: String,
    pub actions: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub service_role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBatchResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub id: Option<String>,
}

/// Create a batch processing task to execute operations on multiple files.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createbatch>
pub struct CreateBatch {
    pub body: CreateBatchBody,
}

impl Ops for CreateBatch {
    const ACTION: &'static str = "CreateBatch";
    type Query = ();
    type Body = CreateBatchBody;
    type Response = BodyResponseProcessor<CreateBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait CreateBatchOps {
    /// Create a batch processing task to execute operations on multiple files.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createbatch>
    fn create_batch(&self, body: CreateBatchBody) -> impl Future<Output = Result<CreateBatchResponse>>;
}

impl CreateBatchOps for Client {
    async fn create_batch(&self, body: CreateBatchBody) -> Result<CreateBatchResponse> {
        self.request(CreateBatch { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBatchQuery {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBatchResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub batch: Option<String>,
}

/// Get batch processing task information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getbatch>
pub struct GetBatch {
    pub query: GetBatchQuery,
}

impl Ops for GetBatch {
    const ACTION: &'static str = "GetBatch";
    type Query = GetBatchQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetBatchOps {
    /// Get batch processing task information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getbatch>
    fn get_batch(&self, query: GetBatchQuery) -> impl Future<Output = Result<GetBatchResponse>>;
}

impl GetBatchOps for Client {
    async fn get_batch(&self, query: GetBatchQuery) -> Result<GetBatchResponse> {
        self.request(GetBatch { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBatchesQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBatchesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub batches: Vec<serde_json::Value>,
    #[serde(default)]
    pub next_token: Option<String>,
}

/// List batch processing tasks with sorting and filtering support.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listbatches>
pub struct ListBatches {
    pub query: ListBatchesQuery,
}

impl Ops for ListBatches {
    const ACTION: &'static str = "ListBatches";
    type Query = ListBatchesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListBatchesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListBatchesOps {
    /// List batch processing tasks with sorting and filtering support.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listbatches>
    fn list_batches(&self, query: ListBatchesQuery) -> impl Future<Output = Result<ListBatchesResponse>>;
}

impl ListBatchesOps for Client {
    async fn list_batches(&self, query: ListBatchesQuery) -> Result<ListBatchesResponse> {
        self.request(ListBatches { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateBatchBody {
    pub project_name: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateBatchResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update batch processing task configuration.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatebatch>
pub struct UpdateBatch {
    pub body: UpdateBatchBody,
}

impl Ops for UpdateBatch {
    const ACTION: &'static str = "UpdateBatch";
    type Query = ();
    type Body = UpdateBatchBody;
    type Response = BodyResponseProcessor<UpdateBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait UpdateBatchOps {
    /// Update batch processing task configuration.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatebatch>
    fn update_batch(&self, body: UpdateBatchBody) -> impl Future<Output = Result<UpdateBatchResponse>>;
}

impl UpdateBatchOps for Client {
    async fn update_batch(&self, body: UpdateBatchBody) -> Result<UpdateBatchResponse> {
        self.request(UpdateBatch { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBatchBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBatchResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a batch processing task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletebatch>
pub struct DeleteBatch {
    pub body: DeleteBatchBody,
}

impl Ops for DeleteBatch {
    const ACTION: &'static str = "DeleteBatch";
    type Query = ();
    type Body = DeleteBatchBody;
    type Response = BodyResponseProcessor<DeleteBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait DeleteBatchOps {
    /// Delete a batch processing task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletebatch>
    fn delete_batch(&self, body: DeleteBatchBody) -> impl Future<Output = Result<DeleteBatchResponse>>;
}

impl DeleteBatchOps for Client {
    async fn delete_batch(&self, body: DeleteBatchBody) -> Result<DeleteBatchResponse> {
        self.request(DeleteBatch { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspendBatchBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspendBatchResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Suspend a running batch processing task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-suspendbatch>
pub struct SuspendBatch {
    pub body: SuspendBatchBody,
}

impl Ops for SuspendBatch {
    const ACTION: &'static str = "SuspendBatch";
    type Query = ();
    type Body = SuspendBatchBody;
    type Response = BodyResponseProcessor<SuspendBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait SuspendBatchOps {
    /// Suspend a running batch processing task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-suspendbatch>
    fn suspend_batch(&self, body: SuspendBatchBody) -> impl Future<Output = Result<SuspendBatchResponse>>;
}

impl SuspendBatchOps for Client {
    async fn suspend_batch(&self, body: SuspendBatchBody) -> Result<SuspendBatchResponse> {
        self.request(SuspendBatch { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeBatchBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeBatchResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Resume a suspended or failed batch processing task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-resumebatch>
pub struct ResumeBatch {
    pub body: ResumeBatchBody,
}

impl Ops for ResumeBatch {
    const ACTION: &'static str = "ResumeBatch";
    type Query = ();
    type Body = ResumeBatchBody;
    type Response = BodyResponseProcessor<ResumeBatchResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait ResumeBatchOps {
    /// Resume a suspended or failed batch processing task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-resumebatch>
    fn resume_batch(&self, body: ResumeBatchBody) -> impl Future<Output = Result<ResumeBatchResponse>>;
}

impl ResumeBatchOps for Client {
    async fn resume_batch(&self, body: ResumeBatchBody) -> Result<ResumeBatchResponse> {
        self.request(ResumeBatch { body }).await
    }
}

pub trait BatchOperations:
    CreateBatchOps
    + GetBatchOps
    + ListBatchesOps
    + UpdateBatchOps
    + DeleteBatchOps
    + SuspendBatchOps
    + ResumeBatchOps
{
}
impl<T> BatchOperations for T where
    T: CreateBatchOps
        + GetBatchOps
        + ListBatchesOps
        + UpdateBatchOps
        + DeleteBatchOps
        + SuspendBatchOps
        + ResumeBatchOps
{
}
