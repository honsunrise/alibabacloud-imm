//! Similar operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSimilarImageClusteringTaskQuery {
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
pub struct CreateSimilarImageClusteringTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a similar image clustering task for image deduplication and selection.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createsimilarimageclusteringtask>
pub struct CreateSimilarImageClusteringTask {
    pub query: CreateSimilarImageClusteringTaskQuery,
}

impl Ops for CreateSimilarImageClusteringTask {
    const ACTION: &'static str = "CreateSimilarImageClusteringTask";
    type Query = CreateSimilarImageClusteringTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateSimilarImageClusteringTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateSimilarImageClusteringTaskOps {
    /// Create a similar image clustering task for image deduplication and selection.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createsimilarimageclusteringtask>
    fn create_similar_image_clustering_task(
        &self,
        query: CreateSimilarImageClusteringTaskQuery,
    ) -> impl Future<Output = Result<CreateSimilarImageClusteringTaskResponse>>;
}

impl CreateSimilarImageClusteringTaskOps for Client {
    async fn create_similar_image_clustering_task(
        &self,
        query: CreateSimilarImageClusteringTaskQuery,
    ) -> Result<CreateSimilarImageClusteringTaskResponse> {
        self.request(CreateSimilarImageClusteringTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySimilarImageClustersQuery {
    pub project_name: String,
    pub dataset_name: String,
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
pub struct QuerySimilarImageClustersResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub similar_image_clusters: Vec<serde_json::Value>,
    #[serde(default)]
    pub next_token: Option<String>,
}

/// Query similar image clusters.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querysimilarimageclusters>
pub struct QuerySimilarImageClusters {
    pub query: QuerySimilarImageClustersQuery,
}

impl Ops for QuerySimilarImageClusters {
    const ACTION: &'static str = "QuerySimilarImageClusters";
    type Query = QuerySimilarImageClustersQuery;
    type Body = ();
    type Response = BodyResponseProcessor<QuerySimilarImageClustersResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait QuerySimilarImageClustersOps {
    /// Query similar image clusters.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querysimilarimageclusters>
    fn query_similar_image_clusters(
        &self,
        query: QuerySimilarImageClustersQuery,
    ) -> impl Future<Output = Result<QuerySimilarImageClustersResponse>>;
}

impl QuerySimilarImageClustersOps for Client {
    async fn query_similar_image_clusters(
        &self,
        query: QuerySimilarImageClustersQuery,
    ) -> Result<QuerySimilarImageClustersResponse> {
        self.request(QuerySimilarImageClusters { query }).await
    }
}

pub trait SimilarOperations: CreateSimilarImageClusteringTaskOps + QuerySimilarImageClustersOps {}
impl<T> SimilarOperations for T where T: CreateSimilarImageClusteringTaskOps + QuerySimilarImageClustersOps {}
