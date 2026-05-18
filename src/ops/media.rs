//! Media operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMediaConvertTaskQuery {
    pub project_name: String,
    pub sources: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMediaConvertTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create an asynchronous media transcoding task for audio/video processing, splicing, and frame extraction.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createmediaconverttask>
pub struct CreateMediaConvertTask {
    pub query: CreateMediaConvertTaskQuery,
}

impl Ops for CreateMediaConvertTask {
    const ACTION: &'static str = "CreateMediaConvertTask";
    type Query = CreateMediaConvertTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateMediaConvertTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateMediaConvertTaskOps {
    /// Create an asynchronous media transcoding task for audio/video processing, splicing, and frame extraction.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createmediaconverttask>
    fn create_media_convert_task(
        &self,
        query: CreateMediaConvertTaskQuery,
    ) -> impl Future<Output = Result<CreateMediaConvertTaskResponse>>;
}

impl CreateMediaConvertTaskOps for Client {
    async fn create_media_convert_task(
        &self,
        query: CreateMediaConvertTaskQuery,
    ) -> Result<CreateMediaConvertTaskResponse> {
        self.request(CreateMediaConvertTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateVideoLabelClassificationTaskQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
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
pub struct CreateVideoLabelClassificationTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a task to detect scene, object, and event labels in video content.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createvideolabelclassificationtask>
pub struct CreateVideoLabelClassificationTask {
    pub query: CreateVideoLabelClassificationTaskQuery,
}

impl Ops for CreateVideoLabelClassificationTask {
    const ACTION: &'static str = "CreateVideoLabelClassificationTask";
    type Query = CreateVideoLabelClassificationTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateVideoLabelClassificationTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateVideoLabelClassificationTaskOps {
    /// Create a task to detect scene, object, and event labels in video content.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createvideolabelclassificationtask>
    fn create_video_label_classification_task(
        &self,
        query: CreateVideoLabelClassificationTaskQuery,
    ) -> impl Future<Output = Result<CreateVideoLabelClassificationTaskResponse>>;
}

impl CreateVideoLabelClassificationTaskOps for Client {
    async fn create_video_label_classification_task(
        &self,
        query: CreateVideoLabelClassificationTaskQuery,
    ) -> Result<CreateVideoLabelClassificationTaskResponse> {
        self.request(CreateVideoLabelClassificationTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetVideoLabelClassificationResultQuery {
    pub project_name: String,
    pub task_type: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetVideoLabelClassificationResultResponse {
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
    pub labels: Vec<serde_json::Value>,
}

/// Get the result of a video label classification task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getvideolabelclassificationresult>
pub struct GetVideoLabelClassificationResult {
    pub query: GetVideoLabelClassificationResultQuery,
}

impl Ops for GetVideoLabelClassificationResult {
    const ACTION: &'static str = "GetVideoLabelClassificationResult";
    type Query = GetVideoLabelClassificationResultQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetVideoLabelClassificationResultResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetVideoLabelClassificationResultOps {
    /// Get the result of a video label classification task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getvideolabelclassificationresult>
    fn get_video_label_classification_result(
        &self,
        query: GetVideoLabelClassificationResultQuery,
    ) -> impl Future<Output = Result<GetVideoLabelClassificationResultResponse>>;
}

impl GetVideoLabelClassificationResultOps for Client {
    async fn get_video_label_classification_result(
        &self,
        query: GetVideoLabelClassificationResultQuery,
    ) -> Result<GetVideoLabelClassificationResultResponse> {
        self.request(GetVideoLabelClassificationResult { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateHighlightTaskQuery {
    pub project_name: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateHighlightTaskBody {
    pub sources: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<serde_json::Value>,
    pub output: serde_json::Value,
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
pub struct CreateHighlightTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a video highlight clipping task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createhighlighttask>
pub struct CreateHighlightTask {
    pub query: CreateHighlightTaskQuery,
    pub body: CreateHighlightTaskBody,
}

impl Ops for CreateHighlightTask {
    const ACTION: &'static str = "CreateHighlightTask";
    type Query = CreateHighlightTaskQuery;
    type Body = CreateHighlightTaskBody;
    type Response = BodyResponseProcessor<CreateHighlightTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait CreateHighlightTaskOps {
    /// Create a video highlight clipping task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createhighlighttask>
    fn create_highlight_task(
        &self,
        query: CreateHighlightTaskQuery,
        body: CreateHighlightTaskBody,
    ) -> impl Future<Output = Result<CreateHighlightTaskResponse>>;
}

impl CreateHighlightTaskOps for Client {
    async fn create_highlight_task(
        &self,
        query: CreateHighlightTaskQuery,
        body: CreateHighlightTaskBody,
    ) -> Result<CreateHighlightTaskResponse> {
        self.request(CreateHighlightTask { query, body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenerateVideoPlaylistQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_start_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subtitles: Option<serde_json::Value>,
    #[serde(rename = "MasterURI", skip_serializing_if = "Option::is_none")]
    pub master_uri: Option<String>,
    pub targets: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenerateVideoPlaylistResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(rename = "MasterURI", default)]
    pub master_uri: Option<String>,
    #[serde(default)]
    pub video_playlist: Vec<serde_json::Value>,
    #[serde(default)]
    pub audio_playlist: Vec<serde_json::Value>,
    #[serde(default)]
    pub subtitle_playlist: Vec<serde_json::Value>,
}

/// Generate an on-the-fly transcoding playlist (m3u8) for instant video playback.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-generatevideoplaylist>
pub struct GenerateVideoPlaylist {
    pub query: GenerateVideoPlaylistQuery,
}

impl Ops for GenerateVideoPlaylist {
    const ACTION: &'static str = "GenerateVideoPlaylist";
    type Query = GenerateVideoPlaylistQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GenerateVideoPlaylistResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GenerateVideoPlaylistOps {
    /// Generate an on-the-fly transcoding playlist (m3u8) for instant video playback.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-generatevideoplaylist>
    fn generate_video_playlist(
        &self,
        query: GenerateVideoPlaylistQuery,
    ) -> impl Future<Output = Result<GenerateVideoPlaylistResponse>>;
}

impl GenerateVideoPlaylistOps for Client {
    async fn generate_video_playlist(
        &self,
        query: GenerateVideoPlaylistQuery,
    ) -> Result<GenerateVideoPlaylistResponse> {
        self.request(GenerateVideoPlaylist { query }).await
    }
}

pub trait MediaOperations:
    CreateMediaConvertTaskOps
    + CreateVideoLabelClassificationTaskOps
    + GetVideoLabelClassificationResultOps
    + CreateHighlightTaskOps
    + GenerateVideoPlaylistOps
{
}
impl<T> MediaOperations for T where
    T: CreateMediaConvertTaskOps
        + CreateVideoLabelClassificationTaskOps
        + GetVideoLabelClassificationResultOps
        + CreateHighlightTaskOps
        + GenerateVideoPlaylistOps
{
}
