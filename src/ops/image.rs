//! Image operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddImageMosaicQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    pub targets: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddImageMosaicResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Add mosaic, Gaussian blur, or solid color blocks to specific regions of an image for privacy protection.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-addimagemosaic>
pub struct AddImageMosaic {
    pub query: AddImageMosaicQuery,
}

impl Ops for AddImageMosaic {
    const ACTION: &'static str = "AddImageMosaic";
    type Query = AddImageMosaicQuery;
    type Body = ();
    type Response = BodyResponseProcessor<AddImageMosaicResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait AddImageMosaicOps {
    /// Add mosaic, Gaussian blur, or solid color blocks to specific regions of an image for privacy protection.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-addimagemosaic>
    fn add_image_mosaic(
        &self,
        query: AddImageMosaicQuery,
    ) -> impl Future<Output = Result<AddImageMosaicResponse>>;
}

impl AddImageMosaicOps for Client {
    async fn add_image_mosaic(&self, query: AddImageMosaicQuery) -> Result<AddImageMosaicResponse> {
        self.request(AddImageMosaic { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompareImageFacesQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompareImageFacesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub similarity: Option<f64>,
}

/// Compare the similarity between the largest faces in two images.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-compareimagefaces>
pub struct CompareImageFaces {
    pub query: CompareImageFacesQuery,
}

impl Ops for CompareImageFaces {
    const ACTION: &'static str = "CompareImageFaces";
    type Query = CompareImageFacesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CompareImageFacesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CompareImageFacesOps {
    /// Compare the similarity between the largest faces in two images.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-compareimagefaces>
    fn compare_image_faces(
        &self,
        query: CompareImageFacesQuery,
    ) -> impl Future<Output = Result<CompareImageFacesResponse>>;
}

impl CompareImageFacesOps for Client {
    async fn compare_image_faces(&self, query: CompareImageFacesQuery) -> Result<CompareImageFacesResponse> {
        self.request(CompareImageFaces { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncodeBlindWatermarkQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_quality: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncodeBlindWatermarkResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Embed invisible watermark text into an image without affecting visual quality.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-encodeblindwatermark>
pub struct EncodeBlindWatermark {
    pub query: EncodeBlindWatermarkQuery,
}

impl Ops for EncodeBlindWatermark {
    const ACTION: &'static str = "EncodeBlindWatermark";
    type Query = EncodeBlindWatermarkQuery;
    type Body = ();
    type Response = BodyResponseProcessor<EncodeBlindWatermarkResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait EncodeBlindWatermarkOps {
    /// Embed invisible watermark text into an image without affecting visual quality.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-encodeblindwatermark>
    fn encode_blind_watermark(
        &self,
        query: EncodeBlindWatermarkQuery,
    ) -> impl Future<Output = Result<EncodeBlindWatermarkResponse>>;
}

impl EncodeBlindWatermarkOps for Client {
    async fn encode_blind_watermark(
        &self,
        query: EncodeBlindWatermarkQuery,
    ) -> Result<EncodeBlindWatermarkResponse> {
        self.request(EncodeBlindWatermark { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageSplicingTaskQuery {
    pub project_name: String,
    pub sources: serde_json::Value,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageSplicingTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a task to splice multiple images into one according to specified rules.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagesplicingtask>
pub struct CreateImageSplicingTask {
    pub query: CreateImageSplicingTaskQuery,
}

impl Ops for CreateImageSplicingTask {
    const ACTION: &'static str = "CreateImageSplicingTask";
    type Query = CreateImageSplicingTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateImageSplicingTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateImageSplicingTaskOps {
    /// Create a task to splice multiple images into one according to specified rules.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagesplicingtask>
    fn create_image_splicing_task(
        &self,
        query: CreateImageSplicingTaskQuery,
    ) -> impl Future<Output = Result<CreateImageSplicingTaskResponse>>;
}

impl CreateImageSplicingTaskOps for Client {
    async fn create_image_splicing_task(
        &self,
        query: CreateImageSplicingTaskQuery,
    ) -> Result<CreateImageSplicingTaskResponse> {
        self.request(CreateImageSplicingTask { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageToPDFTaskQuery {
    pub project_name: String,
    pub sources: serde_json::Value,
    #[serde(rename = "TargetURI")]
    pub target_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageToPDFTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Create a task to convert multiple images into a single PDF file.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagetopdftask>
pub struct CreateImageToPDFTask {
    pub query: CreateImageToPDFTaskQuery,
}

impl Ops for CreateImageToPDFTask {
    const ACTION: &'static str = "CreateImageToPDFTask";
    type Query = CreateImageToPDFTaskQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateImageToPDFTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateImageToPDFTaskOps {
    /// Create a task to convert multiple images into a single PDF file.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createimagetopdftask>
    fn create_image_to_pdf_task(
        &self,
        query: CreateImageToPDFTaskQuery,
    ) -> impl Future<Output = Result<CreateImageToPDFTaskResponse>>;
}

impl CreateImageToPDFTaskOps for Client {
    async fn create_image_to_pdf_task(
        &self,
        query: CreateImageToPDFTaskQuery,
    ) -> Result<CreateImageToPDFTaskResponse> {
        self.request(CreateImageToPDFTask { query }).await
    }
}

pub trait ImageOperations:
    AddImageMosaicOps
    + CompareImageFacesOps
    + EncodeBlindWatermarkOps
    + CreateImageSplicingTaskOps
    + CreateImageToPDFTaskOps
{
}
impl<T> ImageOperations for T where
    T: AddImageMosaicOps
        + CompareImageFacesOps
        + EncodeBlindWatermarkOps
        + CreateImageSplicingTaskOps
        + CreateImageToPDFTaskOps
{
}
