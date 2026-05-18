//! Detection operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageFacesQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageFacesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub faces: Vec<serde_json::Value>,
}

/// Detect faces in an image with boundary, attribute, and quality information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagefaces>
pub struct DetectImageFaces {
    pub query: DetectImageFacesQuery,
}

impl Ops for DetectImageFaces {
    const ACTION: &'static str = "DetectImageFaces";
    type Query = DetectImageFacesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageFacesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageFacesOps {
    /// Detect faces in an image with boundary, attribute, and quality information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagefaces>
    fn detect_image_faces(
        &self,
        query: DetectImageFacesQuery,
    ) -> impl Future<Output = Result<DetectImageFacesResponse>>;
}

impl DetectImageFacesOps for Client {
    async fn detect_image_faces(&self, query: DetectImageFacesQuery) -> Result<DetectImageFacesResponse> {
        self.request(DetectImageFaces { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageBodiesQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageBodiesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub bodies: Vec<serde_json::Value>,
}

/// Detect human bodies in an image, including confidence scores and bounding boxes.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagebodies>
pub struct DetectImageBodies {
    pub query: DetectImageBodiesQuery,
}

impl Ops for DetectImageBodies {
    const ACTION: &'static str = "DetectImageBodies";
    type Query = DetectImageBodiesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageBodiesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageBodiesOps {
    /// Detect human bodies in an image, including confidence scores and bounding boxes.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagebodies>
    fn detect_image_bodies(
        &self,
        query: DetectImageBodiesQuery,
    ) -> impl Future<Output = Result<DetectImageBodiesResponse>>;
}

impl DetectImageBodiesOps for Client {
    async fn detect_image_bodies(&self, query: DetectImageBodiesQuery) -> Result<DetectImageBodiesResponse> {
        self.request(DetectImageBodies { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCarsQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCarsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub cars: Vec<serde_json::Value>,
}

/// Detect vehicles in an image, including car color, type, and license plate information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecars>
pub struct DetectImageCars {
    pub query: DetectImageCarsQuery,
}

impl Ops for DetectImageCars {
    const ACTION: &'static str = "DetectImageCars";
    type Query = DetectImageCarsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageCarsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageCarsOps {
    /// Detect vehicles in an image, including car color, type, and license plate information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecars>
    fn detect_image_cars(
        &self,
        query: DetectImageCarsQuery,
    ) -> impl Future<Output = Result<DetectImageCarsResponse>>;
}

impl DetectImageCarsOps for Client {
    async fn detect_image_cars(&self, query: DetectImageCarsQuery) -> Result<DetectImageCarsResponse> {
        self.request(DetectImageCars { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCodesQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCodesResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub codes: Vec<serde_json::Value>,
}

/// Detect barcodes and QR codes in an image.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecodes>
pub struct DetectImageCodes {
    pub query: DetectImageCodesQuery,
}

impl Ops for DetectImageCodes {
    const ACTION: &'static str = "DetectImageCodes";
    type Query = DetectImageCodesQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageCodesResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageCodesOps {
    /// Detect barcodes and QR codes in an image.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecodes>
    fn detect_image_codes(
        &self,
        query: DetectImageCodesQuery,
    ) -> impl Future<Output = Result<DetectImageCodesResponse>>;
}

impl DetectImageCodesOps for Client {
    async fn detect_image_codes(&self, query: DetectImageCodesQuery) -> Result<DetectImageCodesResponse> {
        self.request(DetectImageCodes { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCroppingQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratios: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageCroppingResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub croppings: Vec<serde_json::Value>,
}

/// Detect visually optimal cropping regions for a given target aspect ratio.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecropping>
pub struct DetectImageCropping {
    pub query: DetectImageCroppingQuery,
}

impl Ops for DetectImageCropping {
    const ACTION: &'static str = "DetectImageCropping";
    type Query = DetectImageCroppingQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageCroppingResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageCroppingOps {
    /// Detect visually optimal cropping regions for a given target aspect ratio.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagecropping>
    fn detect_image_cropping(
        &self,
        query: DetectImageCroppingQuery,
    ) -> impl Future<Output = Result<DetectImageCroppingResponse>>;
}

impl DetectImageCroppingOps for Client {
    async fn detect_image_cropping(
        &self,
        query: DetectImageCroppingQuery,
    ) -> Result<DetectImageCroppingResponse> {
        self.request(DetectImageCropping { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageLabelsQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageLabelsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub labels: Vec<serde_json::Value>,
}

/// Detect scene, object, and event labels in an image.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagelabels>
pub struct DetectImageLabels {
    pub query: DetectImageLabelsQuery,
}

impl Ops for DetectImageLabels {
    const ACTION: &'static str = "DetectImageLabels";
    type Query = DetectImageLabelsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageLabelsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageLabelsOps {
    /// Detect scene, object, and event labels in an image.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagelabels>
    fn detect_image_labels(
        &self,
        query: DetectImageLabelsQuery,
    ) -> impl Future<Output = Result<DetectImageLabelsResponse>>;
}

impl DetectImageLabelsOps for Client {
    async fn detect_image_labels(&self, query: DetectImageLabelsQuery) -> Result<DetectImageLabelsResponse> {
        self.request(DetectImageLabels { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageScoreQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageScoreResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub image_score: Option<serde_json::Value>,
}

/// Evaluate image visual quality based on composition, brightness, contrast, color, and sharpness.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagescore>
pub struct DetectImageScore {
    pub query: DetectImageScoreQuery,
}

impl Ops for DetectImageScore {
    const ACTION: &'static str = "DetectImageScore";
    type Query = DetectImageScoreQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageScoreResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageScoreOps {
    /// Evaluate image visual quality based on composition, brightness, contrast, color, and sharpness.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagescore>
    fn detect_image_score(
        &self,
        query: DetectImageScoreQuery,
    ) -> impl Future<Output = Result<DetectImageScoreResponse>>;
}

impl DetectImageScoreOps for Client {
    async fn detect_image_score(&self, query: DetectImageScoreQuery) -> Result<DetectImageScoreResponse> {
        self.request(DetectImageScore { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageTextsQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectImageTextsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "OCRTexts", default)]
    pub ocr_texts: Option<String>,
    #[serde(rename = "OCRContents", default)]
    pub ocr_contents: Vec<serde_json::Value>,
}

/// Detect and recognize text content in an image (OCR).
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagetexts>
pub struct DetectImageTexts {
    pub query: DetectImageTextsQuery,
}

impl Ops for DetectImageTexts {
    const ACTION: &'static str = "DetectImageTexts";
    type Query = DetectImageTextsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectImageTextsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectImageTextsOps {
    /// Detect and recognize text content in an image (OCR).
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectimagetexts>
    fn detect_image_texts(
        &self,
        query: DetectImageTextsQuery,
    ) -> impl Future<Output = Result<DetectImageTextsResponse>>;
}

impl DetectImageTextsOps for Client {
    async fn detect_image_texts(&self, query: DetectImageTextsQuery) -> Result<DetectImageTextsResponse> {
        self.request(DetectImageTexts { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectMediaMetaQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectMediaMetaResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub video_streams: Vec<serde_json::Value>,
    #[serde(default)]
    pub audio_streams: Vec<serde_json::Value>,
    #[serde(default)]
    pub subtitles: Vec<serde_json::Value>,
    #[serde(default)]
    pub stream_count: Option<i64>,
    #[serde(default)]
    pub program_count: Option<i64>,
    #[serde(default)]
    pub format_name: Option<String>,
    #[serde(default)]
    pub format_long_name: Option<String>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub start_time: Option<f64>,
    #[serde(default)]
    pub bitrate: Option<i64>,
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub album_artist: Option<String>,
    #[serde(default)]
    pub composer: Option<String>,
    #[serde(default)]
    pub performer: Option<String>,
    #[serde(default)]
    pub album: Option<String>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub produce_time: Option<String>,
    #[serde(default)]
    pub lat_long: Option<String>,
    #[serde(default)]
    pub video_width: Option<i64>,
    #[serde(default)]
    pub video_height: Option<i64>,
    #[serde(default)]
    pub addresses: Vec<serde_json::Value>,
}

/// Get media file metadata including format and stream information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectmediameta>
pub struct DetectMediaMeta {
    pub query: DetectMediaMetaQuery,
}

impl Ops for DetectMediaMeta {
    const ACTION: &'static str = "DetectMediaMeta";
    type Query = DetectMediaMetaQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectMediaMetaResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectMediaMetaOps {
    /// Get media file metadata including format and stream information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detectmediameta>
    fn detect_media_meta(
        &self,
        query: DetectMediaMetaQuery,
    ) -> impl Future<Output = Result<DetectMediaMetaResponse>>;
}

impl DetectMediaMetaOps for Client {
    async fn detect_media_meta(&self, query: DetectMediaMetaQuery) -> Result<DetectMediaMetaResponse> {
        self.request(DetectMediaMeta { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectTextAnomalyQuery {
    pub project_name: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetectTextAnomalyResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub suggestion: Option<String>,
}

/// Detect inappropriate content in text including pornography, advertising, spam, and abuse.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detecttextanomaly>
pub struct DetectTextAnomaly {
    pub query: DetectTextAnomalyQuery,
}

impl Ops for DetectTextAnomaly {
    const ACTION: &'static str = "DetectTextAnomaly";
    type Query = DetectTextAnomalyQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DetectTextAnomalyResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DetectTextAnomalyOps {
    /// Detect inappropriate content in text including pornography, advertising, spam, and abuse.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-detecttextanomaly>
    fn detect_text_anomaly(
        &self,
        query: DetectTextAnomalyQuery,
    ) -> impl Future<Output = Result<DetectTextAnomalyResponse>>;
}

impl DetectTextAnomalyOps for Client {
    async fn detect_text_anomaly(&self, query: DetectTextAnomalyQuery) -> Result<DetectTextAnomalyResponse> {
        self.request(DetectTextAnomaly { query }).await
    }
}

pub trait DetectionOperations:
    DetectImageFacesOps
    + DetectImageBodiesOps
    + DetectImageCarsOps
    + DetectImageCodesOps
    + DetectImageCroppingOps
    + DetectImageLabelsOps
    + DetectImageScoreOps
    + DetectImageTextsOps
    + DetectMediaMetaOps
    + DetectTextAnomalyOps
{
}
impl<T> DetectionOperations for T where
    T: DetectImageFacesOps
        + DetectImageBodiesOps
        + DetectImageCarsOps
        + DetectImageCodesOps
        + DetectImageCroppingOps
        + DetectImageLabelsOps
        + DetectImageScoreOps
        + DetectImageTextsOps
        + DetectMediaMetaOps
        + DetectTextAnomalyOps
{
}
