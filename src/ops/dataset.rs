//! Dataset operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDatasetQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_bind_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_file_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_entity_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_relation_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_total_file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDatasetResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub dataset: Option<String>,
}

/// Create a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createdataset>
pub struct CreateDataset {
    pub query: CreateDatasetQuery,
}

impl Ops for CreateDataset {
    const ACTION: &'static str = "CreateDataset";
    type Query = CreateDatasetQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateDatasetResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateDatasetOps {
    /// Create a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createdataset>
    fn create_dataset(
        &self,
        query: CreateDatasetQuery,
    ) -> impl Future<Output = Result<CreateDatasetResponse>>;
}

impl CreateDatasetOps for Client {
    async fn create_dataset(&self, query: CreateDatasetQuery) -> Result<CreateDatasetResponse> {
        self.request(CreateDataset { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDatasetQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_statistics: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDatasetResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub dataset: Option<String>,
}

/// Get dataset information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getdataset>
pub struct GetDataset {
    pub query: GetDatasetQuery,
}

impl Ops for GetDataset {
    const ACTION: &'static str = "GetDataset";
    type Query = GetDatasetQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetDatasetResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetDatasetOps {
    /// Get dataset information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getdataset>
    fn get_dataset(&self, query: GetDatasetQuery) -> impl Future<Output = Result<GetDatasetResponse>>;
}

impl GetDatasetOps for Client {
    async fn get_dataset(&self, query: GetDatasetQuery) -> Result<GetDatasetResponse> {
        self.request(GetDataset { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateDatasetQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_bind_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_file_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_entity_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_relation_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_max_total_file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateDatasetResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub dataset: Option<String>,
}

/// Update dataset information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatedataset>
pub struct UpdateDataset {
    pub query: UpdateDatasetQuery,
}

impl Ops for UpdateDataset {
    const ACTION: &'static str = "UpdateDataset";
    type Query = UpdateDatasetQuery;
    type Body = ();
    type Response = BodyResponseProcessor<UpdateDatasetResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait UpdateDatasetOps {
    /// Update dataset information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatedataset>
    fn update_dataset(
        &self,
        query: UpdateDatasetQuery,
    ) -> impl Future<Output = Result<UpdateDatasetResponse>>;
}

impl UpdateDatasetOps for Client {
    async fn update_dataset(&self, query: UpdateDatasetQuery) -> Result<UpdateDatasetResponse> {
        self.request(UpdateDataset { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDatasetQuery {
    pub project_name: String,
    pub dataset_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDatasetResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletedataset>
pub struct DeleteDataset {
    pub query: DeleteDatasetQuery,
}

impl Ops for DeleteDataset {
    const ACTION: &'static str = "DeleteDataset";
    type Query = DeleteDatasetQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DeleteDatasetResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DeleteDatasetOps {
    /// Delete a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletedataset>
    fn delete_dataset(
        &self,
        query: DeleteDatasetQuery,
    ) -> impl Future<Output = Result<DeleteDatasetResponse>>;
}

impl DeleteDatasetOps for Client {
    async fn delete_dataset(&self, query: DeleteDatasetQuery) -> Result<DeleteDatasetResponse> {
        self.request(DeleteDataset { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDatasetsQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDatasetsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub datasets: Vec<serde_json::Value>,
}

/// List datasets, supporting prefix-based filtering.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listdatasets>
pub struct ListDatasets {
    pub query: ListDatasetsQuery,
}

impl Ops for ListDatasets {
    const ACTION: &'static str = "ListDatasets";
    type Query = ListDatasetsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListDatasetsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListDatasetsOps {
    /// List datasets, supporting prefix-based filtering.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listdatasets>
    fn list_datasets(&self, query: ListDatasetsQuery) -> impl Future<Output = Result<ListDatasetsResponse>>;
}

impl ListDatasetsOps for Client {
    async fn list_datasets(&self, query: ListDatasetsQuery) -> Result<ListDatasetsResponse> {
        self.request(ListDatasets { query }).await
    }
}

pub trait DatasetOperations:
    CreateDatasetOps + GetDatasetOps + UpdateDatasetOps + DeleteDatasetOps + ListDatasetsOps
{
}
impl<T> DatasetOperations for T where
    T: CreateDatasetOps + GetDatasetOps + UpdateDatasetOps + DeleteDatasetOps + ListDatasetsOps
{
}
