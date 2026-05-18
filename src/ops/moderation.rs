//! Moderation operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageModerationTaskQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frames: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageModerationTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a task to detect inappropriate content in images including pornography, terrorism, and violations.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagemoderationtask>
pub struct CreateImageModerationTask {
    pub query: CreateImageModerationTaskQuery,
}

impl Ops for CreateImageModerationTask {
    const ACTION: &'static str = "CreateImageModerationTask";
    type Query = CreateImageModerationTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateImageModerationTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateImageModerationTaskOps {
    /// Create a task to detect inappropriate content in images including pornography, terrorism, and violations.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagemoderationtask>
    fn create_image_moderation_task(
        &self,
        query: CreateImageModerationTaskQuery,
    ) -> impl Future<Output = Result<CreateImageModerationTaskResponse>>;
}

impl CreateImageModerationTaskOps for Client {
    async fn create_image_moderation_task(
        &self,
        query: CreateImageModerationTaskQuery,
    ) -> Result<CreateImageModerationTaskResponse> {
        self.request(CreateImageModerationTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetImageModerationResultQuery {
    pub project_name: String,
    pub task_type: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetImageModerationResultResponse {
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
    pub moderation_result: Option<serde_json::Value>,
}

/// Get the result of an image moderation task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getimagemoderationresult>
pub struct GetImageModerationResult {
    pub query: GetImageModerationResultQuery,
}

impl Ops for GetImageModerationResult {
    const ACTION: &'static str = "GetImageModerationResult";
    type Query = GetImageModerationResultQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetImageModerationResultResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetImageModerationResultOps {
    /// Get the result of an image moderation task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getimagemoderationresult>
    fn get_image_moderation_result(
        &self,
        query: GetImageModerationResultQuery,
    ) -> impl Future<Output = Result<GetImageModerationResultResponse>>;
}

impl GetImageModerationResultOps for Client {
    async fn get_image_moderation_result(
        &self,
        query: GetImageModerationResultQuery,
    ) -> Result<GetImageModerationResultResponse> {
        self.request(GetImageModerationResult { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateVideoModerationTaskQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frames: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateVideoModerationTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a task to detect inappropriate or risky content in videos.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createvideomoderationtask>
pub struct CreateVideoModerationTask {
    pub query: CreateVideoModerationTaskQuery,
}

impl Ops for CreateVideoModerationTask {
    const ACTION: &'static str = "CreateVideoModerationTask";
    type Query = CreateVideoModerationTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateVideoModerationTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateVideoModerationTaskOps {
    /// Create a task to detect inappropriate or risky content in videos.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createvideomoderationtask>
    fn create_video_moderation_task(
        &self,
        query: CreateVideoModerationTaskQuery,
    ) -> impl Future<Output = Result<CreateVideoModerationTaskResponse>>;
}

impl CreateVideoModerationTaskOps for Client {
    async fn create_video_moderation_task(
        &self,
        query: CreateVideoModerationTaskQuery,
    ) -> Result<CreateVideoModerationTaskResponse> {
        self.request(CreateVideoModerationTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetVideoModerationResultQuery {
    pub project_name: String,
    pub task_type: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetVideoModerationResultResponse {
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
    pub moderation_result: Option<serde_json::Value>,
}

/// Get the result of a video moderation task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getvideomoderationresult>
pub struct GetVideoModerationResult {
    pub query: GetVideoModerationResultQuery,
}

impl Ops for GetVideoModerationResult {
    const ACTION: &'static str = "GetVideoModerationResult";
    type Query = GetVideoModerationResultQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetVideoModerationResultResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetVideoModerationResultOps {
    /// Get the result of a video moderation task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getvideomoderationresult>
    fn get_video_moderation_result(
        &self,
        query: GetVideoModerationResultQuery,
    ) -> impl Future<Output = Result<GetVideoModerationResultResponse>>;
}

impl GetVideoModerationResultOps for Client {
    async fn get_video_moderation_result(
        &self,
        query: GetVideoModerationResultQuery,
    ) -> Result<GetVideoModerationResultResponse> {
        self.request(GetVideoModerationResult { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDecodeBlindWatermarkTaskQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark_type: Option<String>,
    #[serde(rename = "TargetURI", skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_quality: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "OriginalImageURI", skip_serializing_if = "Option::is_none")]
    pub original_image_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDecodeBlindWatermarkTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a task to decode blind watermarks from an image.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createdecodeblindwatermarktask>
pub struct CreateDecodeBlindWatermarkTask {
    pub query: CreateDecodeBlindWatermarkTaskQuery,
}

impl Ops for CreateDecodeBlindWatermarkTask {
    const ACTION: &'static str = "CreateDecodeBlindWatermarkTask";
    type Query = CreateDecodeBlindWatermarkTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateDecodeBlindWatermarkTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateDecodeBlindWatermarkTaskOps {
    /// Create a task to decode blind watermarks from an image.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createdecodeblindwatermarktask>
    fn create_decode_blind_watermark_task(
        &self,
        query: CreateDecodeBlindWatermarkTaskQuery,
    ) -> impl Future<Output = Result<CreateDecodeBlindWatermarkTaskResponse>>;
}

impl CreateDecodeBlindWatermarkTaskOps for Client {
    async fn create_decode_blind_watermark_task(
        &self,
        query: CreateDecodeBlindWatermarkTaskQuery,
    ) -> Result<CreateDecodeBlindWatermarkTaskResponse> {
        self.request(CreateDecodeBlindWatermarkTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDecodeBlindWatermarkResultQuery {
    pub project_name: String,
    pub task_type: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDecodeBlindWatermarkResultResponse {
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
    pub content: Option<String>,
}

/// Get the result of a blind watermark decoding task.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getdecodeblindwatermarkresult>
pub struct GetDecodeBlindWatermarkResult {
    pub query: GetDecodeBlindWatermarkResultQuery,
}

impl Ops for GetDecodeBlindWatermarkResult {
    const ACTION: &'static str = "GetDecodeBlindWatermarkResult";
    type Query = GetDecodeBlindWatermarkResultQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetDecodeBlindWatermarkResultResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetDecodeBlindWatermarkResultOps {
    /// Get the result of a blind watermark decoding task.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getdecodeblindwatermarkresult>
    fn get_decode_blind_watermark_result(
        &self,
        query: GetDecodeBlindWatermarkResultQuery,
    ) -> impl Future<Output = Result<GetDecodeBlindWatermarkResultResponse>>;
}

impl GetDecodeBlindWatermarkResultOps for Client {
    async fn get_decode_blind_watermark_result(
        &self,
        query: GetDecodeBlindWatermarkResultQuery,
    ) -> Result<GetDecodeBlindWatermarkResultResponse> {
        self.request(GetDecodeBlindWatermarkResult { query }).await
    }
}

pub trait ModerationOperations:
    CreateImageModerationTaskOps
    + GetImageModerationResultOps
    + CreateVideoModerationTaskOps
    + GetVideoModerationResultOps
    + CreateDecodeBlindWatermarkTaskOps
    + GetDecodeBlindWatermarkResultOps
{
}
impl<T> ModerationOperations for T where
    T: CreateImageModerationTaskOps
        + GetImageModerationResultOps
        + CreateVideoModerationTaskOps
        + GetVideoModerationResultOps
        + CreateDecodeBlindWatermarkTaskOps
        + GetDecodeBlindWatermarkResultOps
{
}
