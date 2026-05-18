//! Project operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_max_dataset_count: Option<i64>,
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
    pub tag: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project: Option<String>,
}

/// Create a project.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createproject>
pub struct CreateProject {
    pub query: CreateProjectQuery,
}

impl Ops for CreateProject {
    const ACTION: &'static str = "CreateProject";
    type Query = CreateProjectQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateProjectResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateProjectOps {
    /// Create a project.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createproject>
    fn create_project(
        &self,
        query: CreateProjectQuery,
    ) -> impl Future<Output = Result<CreateProjectResponse>>;
}

impl CreateProjectOps for Client {
    async fn create_project(&self, query: CreateProjectQuery) -> Result<CreateProjectResponse> {
        self.request(CreateProject { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetProjectQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_statistics: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetProjectResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project: Option<String>,
}

/// Get project information including statistics on datasets and files.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getproject>
pub struct GetProject {
    pub query: GetProjectQuery,
}

impl Ops for GetProject {
    const ACTION: &'static str = "GetProject";
    type Query = GetProjectQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetProjectResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetProjectOps {
    /// Get project information including statistics on datasets and files.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getproject>
    fn get_project(&self, query: GetProjectQuery) -> impl Future<Output = Result<GetProjectResponse>>;
}

impl GetProjectOps for Client {
    async fn get_project(&self, query: GetProjectQuery) -> Result<GetProjectResponse> {
        self.request(GetProject { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateProjectQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_max_dataset_count: Option<i64>,
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
    pub tag: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateProjectResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub project: Option<String>,
}

/// Update project information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updateproject>
pub struct UpdateProject {
    pub query: UpdateProjectQuery,
}

impl Ops for UpdateProject {
    const ACTION: &'static str = "UpdateProject";
    type Query = UpdateProjectQuery;
    type Body = ();
    type Response = BodyResponseProcessor<UpdateProjectResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait UpdateProjectOps {
    /// Update project information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updateproject>
    fn update_project(
        &self,
        query: UpdateProjectQuery,
    ) -> impl Future<Output = Result<UpdateProjectResponse>>;
}

impl UpdateProjectOps for Client {
    async fn update_project(&self, query: UpdateProjectQuery) -> Result<UpdateProjectResponse> {
        self.request(UpdateProject { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteProjectQuery {
    pub project_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteProjectResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a project.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deleteproject>
pub struct DeleteProject {
    pub query: DeleteProjectQuery,
}

impl Ops for DeleteProject {
    const ACTION: &'static str = "DeleteProject";
    type Query = DeleteProjectQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DeleteProjectResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DeleteProjectOps {
    /// Delete a project.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deleteproject>
    fn delete_project(
        &self,
        query: DeleteProjectQuery,
    ) -> impl Future<Output = Result<DeleteProjectResponse>>;
}

impl DeleteProjectOps for Client {
    async fn delete_project(&self, query: DeleteProjectQuery) -> Result<DeleteProjectResponse> {
        self.request(DeleteProject { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListProjectsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListProjectsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub projects: Vec<serde_json::Value>,
}

/// List all projects with their statistics.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listprojects>
pub struct ListProjects {
    pub query: ListProjectsQuery,
}

impl Ops for ListProjects {
    const ACTION: &'static str = "ListProjects";
    type Query = ListProjectsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListProjectsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListProjectsOps {
    /// List all projects with their statistics.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listprojects>
    fn list_projects(&self, query: ListProjectsQuery) -> impl Future<Output = Result<ListProjectsResponse>>;
}

impl ListProjectsOps for Client {
    async fn list_projects(&self, query: ListProjectsQuery) -> Result<ListProjectsResponse> {
        self.request(ListProjects { query }).await
    }
}

pub trait ProjectOperations:
    CreateProjectOps + GetProjectOps + UpdateProjectOps + DeleteProjectOps + ListProjectsOps
{
}
impl<T> ProjectOperations for T where
    T: CreateProjectOps + GetProjectOps + UpdateProjectOps + DeleteProjectOps + ListProjectsOps
{
}
