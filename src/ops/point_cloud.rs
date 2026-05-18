//! Point Cloud operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCompressPointCloudTaskQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_cloud_file_format: Option<String>,
    pub compress_method: String,
    pub point_cloud_fields: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdtree_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub octree_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCompressPointCloudTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a task to compress point cloud data stored in OSS.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createcompresspointcloudtask>
pub struct CreateCompressPointCloudTask {
    pub query: CreateCompressPointCloudTaskQuery,
}

impl Ops for CreateCompressPointCloudTask {
    const ACTION: &'static str = "CreateCompressPointCloudTask";
    type Query = CreateCompressPointCloudTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateCompressPointCloudTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateCompressPointCloudTaskOps {
    /// Create a task to compress point cloud data stored in OSS.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createcompresspointcloudtask>
    fn create_compress_point_cloud_task(
        &self,
        query: CreateCompressPointCloudTaskQuery,
    ) -> impl Future<Output = Result<CreateCompressPointCloudTaskResponse>>;
}

impl CreateCompressPointCloudTaskOps for Client {
    async fn create_compress_point_cloud_task(
        &self,
        query: CreateCompressPointCloudTaskQuery,
    ) -> Result<CreateCompressPointCloudTaskResponse> {
        self.request(CreateCompressPointCloudTask { query }).await
    }
}
