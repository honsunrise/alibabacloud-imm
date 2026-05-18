//! Document operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateOfficeConversionTaskQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(rename = "TargetURI", skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    #[serde(rename = "TargetURIPrefix", skip_serializing_if = "Option::is_none")]
    pub target_uri_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    pub target_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_percentage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sheet_row: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sheet_column: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit_to_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit_to_height: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_horizontal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_picture: Option<bool>,
    #[serde(rename = "ImageDPI", skip_serializing_if = "Option::is_none")]
    pub image_dpi: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_line_feed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateOfficeConversionTaskBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateOfficeConversionTaskResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}

/// Create a document conversion task to convert Office documents to images, text, or PDF.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createofficeconversiontask>
pub struct CreateOfficeConversionTask {
    pub query: CreateOfficeConversionTaskQuery,
    pub body: CreateOfficeConversionTaskBody,
}

impl Ops for CreateOfficeConversionTask {
    const ACTION: &'static str = "CreateOfficeConversionTask";
    type Query = CreateOfficeConversionTaskQuery;
    type Body = CreateOfficeConversionTaskBody;
    type Response = BodyResponseProcessor<CreateOfficeConversionTaskResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait CreateOfficeConversionTaskOps {
    /// Create a document conversion task to convert Office documents to images, text, or PDF.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createofficeconversiontask>
    fn create_office_conversion_task(
        &self,
        query: CreateOfficeConversionTaskQuery,
        body: CreateOfficeConversionTaskBody,
    ) -> impl Future<Output = Result<CreateOfficeConversionTaskResponse>>;
}

impl CreateOfficeConversionTaskOps for Client {
    async fn create_office_conversion_task(
        &self,
        query: CreateOfficeConversionTaskQuery,
        body: CreateOfficeConversionTaskBody,
    ) -> Result<CreateOfficeConversionTaskResponse> {
        self.request(CreateOfficeConversionTask { query, body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenerateWebofficeTokenQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_pages: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_uploaded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_topic_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidecmb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenerateWebofficeTokenResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "WebofficeURL", default)]
    pub weboffice_url: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub access_token_expired_time: Option<String>,
    #[serde(default)]
    pub refresh_token_expired_time: Option<String>,
}

/// Generate a WebOffice preview/edit token for document collaboration.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-generatewebofficetoken>
pub struct GenerateWebofficeToken {
    pub query: GenerateWebofficeTokenQuery,
}

impl Ops for GenerateWebofficeToken {
    const ACTION: &'static str = "GenerateWebofficeToken";
    type Query = GenerateWebofficeTokenQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GenerateWebofficeTokenResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GenerateWebofficeTokenOps {
    /// Generate a WebOffice preview/edit token for document collaboration.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-generatewebofficetoken>
    fn generate_weboffice_token(
        &self,
        query: GenerateWebofficeTokenQuery,
    ) -> impl Future<Output = Result<GenerateWebofficeTokenResponse>>;
}

impl GenerateWebofficeTokenOps for Client {
    async fn generate_weboffice_token(
        &self,
        query: GenerateWebofficeTokenQuery,
    ) -> Result<GenerateWebofficeTokenResponse> {
        self.request(GenerateWebofficeToken { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefreshWebofficeTokenQuery {
    pub project_name: String,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefreshWebofficeTokenResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token_expired_time: Option<String>,
    #[serde(default)]
    pub access_token_expired_time: Option<String>,
}

/// Refresh a WebOffice access token before it expires.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-refreshwebofficetoken>
pub struct RefreshWebofficeToken {
    pub query: RefreshWebofficeTokenQuery,
}

impl Ops for RefreshWebofficeToken {
    const ACTION: &'static str = "RefreshWebofficeToken";
    type Query = RefreshWebofficeTokenQuery;
    type Body = ();
    type Response = BodyResponseProcessor<RefreshWebofficeTokenResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait RefreshWebofficeTokenOps {
    /// Refresh a WebOffice access token before it expires.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-refreshwebofficetoken>
    fn refresh_weboffice_token(
        &self,
        query: RefreshWebofficeTokenQuery,
    ) -> impl Future<Output = Result<RefreshWebofficeTokenResponse>>;
}

impl RefreshWebofficeTokenOps for Client {
    async fn refresh_weboffice_token(
        &self,
        query: RefreshWebofficeTokenQuery,
    ) -> Result<RefreshWebofficeTokenResponse> {
        self.request(RefreshWebofficeToken { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtractDocumentTextQuery {
    pub project_name: String,
    #[serde(rename = "SourceURI")]
    pub source_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtractDocumentTextResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub document_text: Option<String>,
}

/// Extract text content from a document file.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-extractdocumenttext>
pub struct ExtractDocumentText {
    pub query: ExtractDocumentTextQuery,
}

impl Ops for ExtractDocumentText {
    const ACTION: &'static str = "ExtractDocumentText";
    type Query = ExtractDocumentTextQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ExtractDocumentTextResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ExtractDocumentTextOps {
    /// Extract text content from a document file.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-extractdocumenttext>
    fn extract_document_text(
        &self,
        query: ExtractDocumentTextQuery,
    ) -> impl Future<Output = Result<ExtractDocumentTextResponse>>;
}

impl ExtractDocumentTextOps for Client {
    async fn extract_document_text(
        &self,
        query: ExtractDocumentTextQuery,
    ) -> Result<ExtractDocumentTextResponse> {
        self.request(ExtractDocumentText { query }).await
    }
}

pub trait DocumentOperations:
    CreateOfficeConversionTaskOps + GenerateWebofficeTokenOps + RefreshWebofficeTokenOps + ExtractDocumentTextOps
{
}
impl<T> DocumentOperations for T where
    T: CreateOfficeConversionTaskOps
        + GenerateWebofficeTokenOps
        + RefreshWebofficeTokenOps
        + ExtractDocumentTextOps
{
}
