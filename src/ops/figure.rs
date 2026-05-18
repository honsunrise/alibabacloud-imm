//! Figure operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFigureClusteringTaskQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFigureClusteringTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a figure clustering task to group faces of different people in indexed images.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfigureclusteringtask>
pub struct CreateFigureClusteringTask {
    pub query: CreateFigureClusteringTaskQuery,
}

impl Ops for CreateFigureClusteringTask {
    const ACTION: &'static str = "CreateFigureClusteringTask";
    type Query = CreateFigureClusteringTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateFigureClusteringTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateFigureClusteringTaskOps {
    /// Create a figure clustering task to group faces of different people in indexed images.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfigureclusteringtask>
    fn create_figure_clustering_task(
        &self,
        query: CreateFigureClusteringTaskQuery,
    ) -> impl Future<Output = Result<CreateFigureClusteringTaskResponse>>;
}

impl CreateFigureClusteringTaskOps for Client {
    async fn create_figure_clustering_task(
        &self,
        query: CreateFigureClusteringTaskQuery,
    ) -> Result<CreateFigureClusteringTaskResponse> {
        self.request(CreateFigureClusteringTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFigureClustersMergingTaskQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "From", skip_serializing_if = "Option::is_none")]
    pub from_field: Option<String>,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub froms: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFigureClustersMergingTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Merge two or more figure clusters into one.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfigureclustersmergingtask>
pub struct CreateFigureClustersMergingTask {
    pub query: CreateFigureClustersMergingTaskQuery,
}

impl Ops for CreateFigureClustersMergingTask {
    const ACTION: &'static str = "CreateFigureClustersMergingTask";
    type Query = CreateFigureClustersMergingTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateFigureClustersMergingTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateFigureClustersMergingTaskOps {
    /// Merge two or more figure clusters into one.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfigureclustersmergingtask>
    fn create_figure_clusters_merging_task(
        &self,
        query: CreateFigureClustersMergingTaskQuery,
    ) -> impl Future<Output = Result<CreateFigureClustersMergingTaskResponse>>;
}

impl CreateFigureClustersMergingTaskOps for Client {
    async fn create_figure_clusters_merging_task(
        &self,
        query: CreateFigureClustersMergingTaskQuery,
    ) -> Result<CreateFigureClustersMergingTaskResponse> {
        self.request(CreateFigureClustersMergingTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFigureClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFigureClusterResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub figure_cluster: Option<String>,
}

/// Get figure cluster information including creation time, photo count, and cover.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getfigurecluster>
pub struct GetFigureCluster {
    pub query: GetFigureClusterQuery,
}

impl Ops for GetFigureCluster {
    const ACTION: &'static str = "GetFigureCluster";
    type Query = GetFigureClusterQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetFigureClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetFigureClusterOps {
    /// Get figure cluster information including creation time, photo count, and cover.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getfigurecluster>
    fn get_figure_cluster(
        &self,
        query: GetFigureClusterQuery,
    ) -> impl Future<Output = Result<GetFigureClusterResponse>>;
}

impl GetFigureClusterOps for Client {
    async fn get_figure_cluster(&self, query: GetFigureClusterQuery) -> Result<GetFigureClusterResponse> {
        self.request(GetFigureCluster { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchGetFigureClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub object_ids: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchGetFigureClusterResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub figure_clusters: Vec<serde_json::Value>,
}

/// Get figure clusters and their information in batch.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchgetfigurecluster>
pub struct BatchGetFigureCluster {
    pub query: BatchGetFigureClusterQuery,
}

impl Ops for BatchGetFigureCluster {
    const ACTION: &'static str = "BatchGetFigureCluster";
    type Query = BatchGetFigureClusterQuery;
    type Body = ();
    type Response = BodyResponseProcessor<BatchGetFigureClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait BatchGetFigureClusterOps {
    /// Get figure clusters and their information in batch.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-batchgetfigurecluster>
    fn batch_get_figure_cluster(
        &self,
        query: BatchGetFigureClusterQuery,
    ) -> impl Future<Output = Result<BatchGetFigureClusterResponse>>;
}

impl BatchGetFigureClusterOps for Client {
    async fn batch_get_figure_cluster(
        &self,
        query: BatchGetFigureClusterQuery,
    ) -> Result<BatchGetFigureClusterResponse> {
        self.request(BatchGetFigureCluster { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryFigureClustersQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_total_count: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryFigureClustersResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub figure_clusters: Vec<serde_json::Value>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

/// Query figure clusters by various conditions.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-queryfigureclusters>
pub struct QueryFigureClusters {
    pub query: QueryFigureClustersQuery,
}

impl Ops for QueryFigureClusters {
    const ACTION: &'static str = "QueryFigureClusters";
    type Query = QueryFigureClustersQuery;
    type Body = ();
    type Response = BodyResponseProcessor<QueryFigureClustersResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait QueryFigureClustersOps {
    /// Query figure clusters by various conditions.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-queryfigureclusters>
    fn query_figure_clusters(
        &self,
        query: QueryFigureClustersQuery,
    ) -> impl Future<Output = Result<QueryFigureClustersResponse>>;
}

impl QueryFigureClustersOps for Client {
    async fn query_figure_clusters(
        &self,
        query: QueryFigureClustersQuery,
    ) -> Result<QueryFigureClustersResponse> {
        self.request(QueryFigureClusters { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFigureClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub figure_cluster: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFigureClusterResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update figure cluster information such as name and labels.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatefigurecluster>
pub struct UpdateFigureCluster {
    pub query: UpdateFigureClusterQuery,
}

impl Ops for UpdateFigureCluster {
    const ACTION: &'static str = "UpdateFigureCluster";
    type Query = UpdateFigureClusterQuery;
    type Body = ();
    type Response = BodyResponseProcessor<UpdateFigureClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait UpdateFigureClusterOps {
    /// Update figure cluster information such as name and labels.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatefigurecluster>
    fn update_figure_cluster(
        &self,
        query: UpdateFigureClusterQuery,
    ) -> impl Future<Output = Result<UpdateFigureClusterResponse>>;
}

impl UpdateFigureClusterOps for Client {
    async fn update_figure_cluster(
        &self,
        query: UpdateFigureClusterQuery,
    ) -> Result<UpdateFigureClusterResponse> {
        self.request(UpdateFigureCluster { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchImageFigureClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchImageFigureClusterResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub clusters: Vec<serde_json::Value>,
}

/// Search for figure clusters that contain faces from a specified image.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-searchimagefigurecluster>
pub struct SearchImageFigureCluster {
    pub query: SearchImageFigureClusterQuery,
}

impl Ops for SearchImageFigureCluster {
    const ACTION: &'static str = "SearchImageFigureCluster";
    type Query = SearchImageFigureClusterQuery;
    type Body = ();
    type Response = BodyResponseProcessor<SearchImageFigureClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait SearchImageFigureClusterOps {
    /// Search for figure clusters that contain faces from a specified image.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-searchimagefigurecluster>
    fn search_image_figure_cluster(
        &self,
        query: SearchImageFigureClusterQuery,
    ) -> impl Future<Output = Result<SearchImageFigureClusterResponse>>;
}

impl SearchImageFigureClusterOps for Client {
    async fn search_image_figure_cluster(
        &self,
        query: SearchImageFigureClusterQuery,
    ) -> Result<SearchImageFigureClusterResponse> {
        self.request(SearchImageFigureCluster { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFacesSearchingTaskQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFacesSearchingTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a task to search for the most similar faces in a dataset.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfacessearchingtask>
pub struct CreateFacesSearchingTask {
    pub query: CreateFacesSearchingTaskQuery,
}

impl Ops for CreateFacesSearchingTask {
    const ACTION: &'static str = "CreateFacesSearchingTask";
    type Query = CreateFacesSearchingTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateFacesSearchingTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateFacesSearchingTaskOps {
    /// Create a task to search for the most similar faces in a dataset.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createfacessearchingtask>
    fn create_faces_searching_task(
        &self,
        query: CreateFacesSearchingTaskQuery,
    ) -> impl Future<Output = Result<CreateFacesSearchingTaskResponse>>;
}

impl CreateFacesSearchingTaskOps for Client {
    async fn create_faces_searching_task(
        &self,
        query: CreateFacesSearchingTaskQuery,
    ) -> Result<CreateFacesSearchingTaskResponse> {
        self.request(CreateFacesSearchingTask { query }).await
    }
}

pub trait FigureOperations:
    CreateFigureClusteringTaskOps
    + CreateFigureClustersMergingTaskOps
    + GetFigureClusterOps
    + BatchGetFigureClusterOps
    + QueryFigureClustersOps
    + UpdateFigureClusterOps
    + SearchImageFigureClusterOps
    + CreateFacesSearchingTaskOps
{
}
impl<T> FigureOperations for T where
    T: CreateFigureClusteringTaskOps
        + CreateFigureClustersMergingTaskOps
        + GetFigureClusterOps
        + BatchGetFigureClusterOps
        + QueryFigureClustersOps
        + UpdateFigureClusterOps
        + SearchImageFigureClusterOps
        + CreateFacesSearchingTaskOps
{
}
