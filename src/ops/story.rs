//! Story operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateStoryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateStoryBody {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    pub story_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_file_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_topic_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateStoryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a story.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createstory>
pub struct CreateStory {
    pub query: CreateStoryQuery,
    pub body: CreateStoryBody,
}

impl Ops for CreateStory {
    const ACTION: &'static str = "CreateStory";
    type Query = CreateStoryQuery;
    type Body = CreateStoryBody;
    type Response = BodyResponseProcessor<CreateStoryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait CreateStoryOps {
    /// Create a story.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createstory>
    fn create_story(
        &self,
        query: CreateStoryQuery,
        body: CreateStoryBody,
    ) -> impl Future<Output = Result<CreateStoryResponse>>;
}

impl CreateStoryOps for Client {
    async fn create_story(
        &self,
        query: CreateStoryQuery,
        body: CreateStoryBody,
    ) -> Result<CreateStoryResponse> {
        self.request(CreateStory { query, body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCustomizedStoryBody {
    pub project_name: String,
    pub dataset_name: String,
    pub story_type: String,
    pub story_sub_type: String,
    pub story_name: String,
    pub cover: serde_json::Value,
    pub files: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCustomizedStoryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub object_id: Option<String>,
}

/// Create a customized story using selected images and videos.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createcustomizedstory>
pub struct CreateCustomizedStory {
    pub body: CreateCustomizedStoryBody,
}

impl Ops for CreateCustomizedStory {
    const ACTION: &'static str = "CreateCustomizedStory";
    type Query = ();
    type Body = CreateCustomizedStoryBody;
    type Response = BodyResponseProcessor<CreateCustomizedStoryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait CreateCustomizedStoryOps {
    /// Create a customized story using selected images and videos.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createcustomizedstory>
    fn create_customized_story(
        &self,
        body: CreateCustomizedStoryBody,
    ) -> impl Future<Output = Result<CreateCustomizedStoryResponse>>;
}

impl CreateCustomizedStoryOps for Client {
    async fn create_customized_story(
        &self,
        body: CreateCustomizedStoryBody,
    ) -> Result<CreateCustomizedStoryResponse> {
        self.request(CreateCustomizedStory { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetStoryQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetStoryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub story: Option<String>,
}

/// Get story information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getstory>
pub struct GetStory {
    pub query: GetStoryQuery,
}

impl Ops for GetStory {
    const ACTION: &'static str = "GetStory";
    type Query = GetStoryQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetStoryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetStoryOps {
    /// Get story information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getstory>
    fn get_story(&self, query: GetStoryQuery) -> impl Future<Output = Result<GetStoryResponse>>;
}

impl GetStoryOps for Client {
    async fn get_story(&self, query: GetStoryQuery) -> Result<GetStoryResponse> {
        self.request(GetStory { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryStoriesQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figure_cluster_ids: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_start_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_end_time_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_empty_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryStoriesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub stories: Vec<serde_json::Value>,
    #[serde(default)]
    pub next_token: Option<String>,
}

/// Query stories by various conditions.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querystories>
pub struct QueryStories {
    pub query: QueryStoriesQuery,
}

impl Ops for QueryStories {
    const ACTION: &'static str = "QueryStories";
    type Query = QueryStoriesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<QueryStoriesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait QueryStoriesOps {
    /// Query stories by various conditions.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-querystories>
    fn query_stories(&self, query: QueryStoriesQuery) -> impl Future<Output = Result<QueryStoriesResponse>>;
}

impl QueryStoriesOps for Client {
    async fn query_stories(&self, query: QueryStoriesQuery) -> Result<QueryStoriesResponse> {
        self.request(QueryStories { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStoryBody {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStoryResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update story information such as name, cover, and labels.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatestory>
pub struct UpdateStory {
    pub body: UpdateStoryBody,
}

impl Ops for UpdateStory {
    const ACTION: &'static str = "UpdateStory";
    type Query = ();
    type Body = UpdateStoryBody;
    type Response = BodyResponseProcessor<UpdateStoryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait UpdateStoryOps {
    /// Update story information such as name, cover, and labels.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatestory>
    fn update_story(&self, body: UpdateStoryBody) -> impl Future<Output = Result<UpdateStoryResponse>>;
}

impl UpdateStoryOps for Client {
    async fn update_story(&self, body: UpdateStoryBody) -> Result<UpdateStoryResponse> {
        self.request(UpdateStory { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteStoryQuery {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteStoryResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a story.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletestory>
pub struct DeleteStory {
    pub query: DeleteStoryQuery,
}

impl Ops for DeleteStory {
    const ACTION: &'static str = "DeleteStory";
    type Query = DeleteStoryQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DeleteStoryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DeleteStoryOps {
    /// Delete a story.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletestory>
    fn delete_story(&self, query: DeleteStoryQuery) -> impl Future<Output = Result<DeleteStoryResponse>>;
}

impl DeleteStoryOps for Client {
    async fn delete_story(&self, query: DeleteStoryQuery) -> Result<DeleteStoryResponse> {
        self.request(DeleteStory { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddStoryFilesBody {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
    pub files: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddStoryFilesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
}

/// Add files to a story.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-addstoryfiles>
pub struct AddStoryFiles {
    pub body: AddStoryFilesBody,
}

impl Ops for AddStoryFiles {
    const ACTION: &'static str = "AddStoryFiles";
    type Query = ();
    type Body = AddStoryFilesBody;
    type Response = BodyResponseProcessor<AddStoryFilesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait AddStoryFilesOps {
    /// Add files to a story.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-addstoryfiles>
    fn add_story_files(&self, body: AddStoryFilesBody)
    -> impl Future<Output = Result<AddStoryFilesResponse>>;
}

impl AddStoryFilesOps for Client {
    async fn add_story_files(&self, body: AddStoryFilesBody) -> Result<AddStoryFilesResponse> {
        self.request(AddStoryFiles { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveStoryFilesBody {
    pub project_name: String,
    pub dataset_name: String,
    pub object_id: String,
    pub files: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveStoryFilesResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Remove files from a story.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-removestoryfiles>
pub struct RemoveStoryFiles {
    pub body: RemoveStoryFilesBody,
}

impl Ops for RemoveStoryFiles {
    const ACTION: &'static str = "RemoveStoryFiles";
    type Query = ();
    type Body = RemoveStoryFilesBody;
    type Response = BodyResponseProcessor<RemoveStoryFilesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait RemoveStoryFilesOps {
    /// Remove files from a story.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-removestoryfiles>
    fn remove_story_files(
        &self,
        body: RemoveStoryFilesBody,
    ) -> impl Future<Output = Result<RemoveStoryFilesResponse>>;
}

impl RemoveStoryFilesOps for Client {
    async fn remove_story_files(&self, body: RemoveStoryFilesBody) -> Result<RemoveStoryFilesResponse> {
        self.request(RemoveStoryFiles { body }).await
    }
}

pub trait StoryOperations:
    CreateStoryOps
    + CreateCustomizedStoryOps
    + GetStoryOps
    + QueryStoriesOps
    + UpdateStoryOps
    + DeleteStoryOps
    + AddStoryFilesOps
    + RemoveStoryFilesOps
{
}
impl<T> StoryOperations for T where
    T: CreateStoryOps
        + CreateCustomizedStoryOps
        + GetStoryOps
        + QueryStoriesOps
        + UpdateStoryOps
        + DeleteStoryOps
        + AddStoryFilesOps
        + RemoveStoryFilesOps
{
}
