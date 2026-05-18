//! Task operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTaskQuery {
    pub project_name: String,
    pub task_type: String,
    pub task_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_definition: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub user_data: Option<String>,
    #[serde(default)]
    pub task_type: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub tags: Option<serde_json::Value>,
    #[serde(default)]
    pub task_request_definition: Option<String>,
    #[serde(default)]
    pub progress: Option<i64>,
}

/// Get asynchronous task information by task ID and type.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-gettask>
pub struct GetTask {
    pub query: GetTaskQuery,
}

impl Ops for GetTask {
    const ACTION: &'static str = "GetTask";
    type Query = GetTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetTaskOps {
    /// Get asynchronous task information by task ID and type.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-gettask>
    fn get_task(&self, query: GetTaskQuery) -> impl Future<Output = Result<GetTaskResponse>>;
}

impl GetTaskOps for Client {
    async fn get_task(&self, query: GetTaskQuery) -> Result<GetTaskResponse> {
        self.request(GetTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTasksQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_types: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_definition: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTasksResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub max_results: Option<String>,
    #[serde(default)]
    pub tasks: Vec<serde_json::Value>,
}

/// List tasks with filtering by time range, status, type, and tags.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listtasks>
pub struct ListTasks {
    pub query: ListTasksQuery,
}

impl Ops for ListTasks {
    const ACTION: &'static str = "ListTasks";
    type Query = ListTasksQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListTasksResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListTasksOps {
    /// List tasks with filtering by time range, status, type, and tags.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listtasks>
    fn list_tasks(&self, query: ListTasksQuery) -> impl Future<Output = Result<ListTasksResponse>>;
}

impl ListTasksOps for Client {
    async fn list_tasks(&self, query: ListTasksQuery) -> Result<ListTasksResponse> {
        self.request(ListTasks { query }).await
    }
}

pub trait TaskOperations: GetTaskOps + ListTasksOps {}
impl<T> TaskOperations for T where T: GetTaskOps + ListTasksOps {}
