//! Location operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateLocationDateClusteringTaskQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub date_options: serde_json::Value,
    pub location_options: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateLocationDateClusteringTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a location-date clustering task to group files by time and geographic location.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createlocationdateclusteringtask>
pub struct CreateLocationDateClusteringTask {
    pub query: CreateLocationDateClusteringTaskQuery,
}

impl Ops for CreateLocationDateClusteringTask {
    const ACTION: &'static str = "CreateLocationDateClusteringTask";
    type Query = CreateLocationDateClusteringTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateLocationDateClusteringTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateLocationDateClusteringTaskOps {
    /// Create a location-date clustering task to group files by time and geographic location.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createlocationdateclusteringtask>
    fn create_location_date_clustering_task(
        &self,
        query: CreateLocationDateClusteringTaskQuery,
    ) -> impl Future<Output = Result<CreateLocationDateClusteringTaskResponse>>;
}

impl CreateLocationDateClusteringTaskOps for Client {
    async fn create_location_date_clustering_task(
        &self,
        query: CreateLocationDateClusteringTaskQuery,
    ) -> Result<CreateLocationDateClusteringTaskResponse> {
        self.request(CreateLocationDateClusteringTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryLocationDateClustersQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_date_cluster_levels: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_date_cluster_start_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_date_cluster_end_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryLocationDateClustersResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub location_date_clusters: Vec<serde_json::Value>,
    #[serde(default)]
    pub next_token: Option<String>,
}

/// Query location-date clusters with multiple filter conditions.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querylocationdateclusters>
pub struct QueryLocationDateClusters {
    pub query: QueryLocationDateClustersQuery,
}

impl Ops for QueryLocationDateClusters {
    const ACTION: &'static str = "QueryLocationDateClusters";
    type Query = QueryLocationDateClustersQuery;
    type Body = ();
    type Response = BodyResponseProcessor<QueryLocationDateClustersResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait QueryLocationDateClustersOps {
    /// Query location-date clusters with multiple filter conditions.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querylocationdateclusters>
    fn query_location_date_clusters(
        &self,
        query: QueryLocationDateClustersQuery,
    ) -> impl Future<Output = Result<QueryLocationDateClustersResponse>>;
}

impl QueryLocationDateClustersOps for Client {
    async fn query_location_date_clusters(
        &self,
        query: QueryLocationDateClustersQuery,
    ) -> Result<QueryLocationDateClustersResponse> {
        self.request(QueryLocationDateClusters { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateLocationDateClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateLocationDateClusterResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update a location-date cluster.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatelocationdatecluster>
pub struct UpdateLocationDateCluster {
    pub query: UpdateLocationDateClusterQuery,
}

impl Ops for UpdateLocationDateCluster {
    const ACTION: &'static str = "UpdateLocationDateCluster";
    type Query = UpdateLocationDateClusterQuery;
    type Body = ();
    type Response = BodyResponseProcessor<UpdateLocationDateClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait UpdateLocationDateClusterOps {
    /// Update a location-date cluster.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatelocationdatecluster>
    fn update_location_date_cluster(
        &self,
        query: UpdateLocationDateClusterQuery,
    ) -> impl Future<Output = Result<UpdateLocationDateClusterResponse>>;
}

impl UpdateLocationDateClusterOps for Client {
    async fn update_location_date_cluster(
        &self,
        query: UpdateLocationDateClusterQuery,
    ) -> Result<UpdateLocationDateClusterResponse> {
        self.request(UpdateLocationDateCluster { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteLocationDateClusterQuery {
    pub project_name: String,
    pub dataset_name: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteLocationDateClusterBody {
    pub object_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteLocationDateClusterResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a location-date cluster.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletelocationdatecluster>
pub struct DeleteLocationDateCluster {
    pub query: DeleteLocationDateClusterQuery,
    pub body: DeleteLocationDateClusterBody,
}

impl Ops for DeleteLocationDateCluster {
    const ACTION: &'static str = "DeleteLocationDateCluster";
    type Query = DeleteLocationDateClusterQuery;
    type Body = DeleteLocationDateClusterBody;
    type Response = BodyResponseProcessor<DeleteLocationDateClusterResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait DeleteLocationDateClusterOps {
    /// Delete a location-date cluster.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletelocationdatecluster>
    fn delete_location_date_cluster(
        &self,
        query: DeleteLocationDateClusterQuery,
        body: DeleteLocationDateClusterBody,
    ) -> impl Future<Output = Result<DeleteLocationDateClusterResponse>>;
}

impl DeleteLocationDateClusterOps for Client {
    async fn delete_location_date_cluster(
        &self,
        query: DeleteLocationDateClusterQuery,
        body: DeleteLocationDateClusterBody,
    ) -> Result<DeleteLocationDateClusterResponse> {
        self.request(DeleteLocationDateCluster { query, body }).await
    }
}

pub trait LocationOperations:
    CreateLocationDateClusteringTaskOps
    + QueryLocationDateClustersOps
    + UpdateLocationDateClusterOps
    + DeleteLocationDateClusterOps
{
}
impl<T> LocationOperations for T where
    T: CreateLocationDateClusteringTaskOps
        + QueryLocationDateClustersOps
        + UpdateLocationDateClusterOps
        + DeleteLocationDateClusterOps
{
}
