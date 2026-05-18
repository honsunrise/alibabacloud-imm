//! Binding operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBindingQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "URI")]
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBindingResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub binding: Option<String>,
}

/// Create a binding between a dataset and an OSS bucket for automatic file synchronization and indexing.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createbinding>
pub struct CreateBinding {
    pub query: CreateBindingQuery,
}

impl Ops for CreateBinding {
    const ACTION: &'static str = "CreateBinding";
    type Query = CreateBindingQuery;
    type Body = ();
    type Response = BodyResponseProcessor<CreateBindingResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait CreateBindingOps {
    /// Create a binding between a dataset and an OSS bucket for automatic file synchronization and indexing.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createbinding>
    fn create_binding(
        &self,
        query: CreateBindingQuery,
    ) -> impl Future<Output = Result<CreateBindingResponse>>;
}

impl CreateBindingOps for Client {
    async fn create_binding(&self, query: CreateBindingQuery) -> Result<CreateBindingResponse> {
        self.request(CreateBinding { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBindingQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "URI", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBindingResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub binding: Option<String>,
}

/// Get binding relationship details between a dataset and an OSS bucket.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getbinding>
pub struct GetBinding {
    pub query: GetBindingQuery,
}

impl Ops for GetBinding {
    const ACTION: &'static str = "GetBinding";
    type Query = GetBindingQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetBindingResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetBindingOps {
    /// Get binding relationship details between a dataset and an OSS bucket.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-getbinding>
    fn get_binding(&self, query: GetBindingQuery) -> impl Future<Output = Result<GetBindingResponse>>;
}

impl GetBindingOps for Client {
    async fn get_binding(&self, query: GetBindingQuery) -> Result<GetBindingResponse> {
        self.request(GetBinding { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBindingQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "URI", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBindingResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete the binding between a dataset and an OSS bucket.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletebinding>
pub struct DeleteBinding {
    pub query: DeleteBindingQuery,
}

impl Ops for DeleteBinding {
    const ACTION: &'static str = "DeleteBinding";
    type Query = DeleteBindingQuery;
    type Body = ();
    type Response = BodyResponseProcessor<DeleteBindingResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait DeleteBindingOps {
    /// Delete the binding between a dataset and an OSS bucket.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletebinding>
    fn delete_binding(
        &self,
        query: DeleteBindingQuery,
    ) -> impl Future<Output = Result<DeleteBindingResponse>>;
}

impl DeleteBindingOps for Client {
    async fn delete_binding(&self, query: DeleteBindingQuery) -> Result<DeleteBindingResponse> {
        self.request(DeleteBinding { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBindingsQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBindingsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub bindings: Vec<serde_json::Value>,
}

/// List binding relationships between datasets and OSS buckets.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listbindings>
pub struct ListBindings {
    pub query: ListBindingsQuery,
}

impl Ops for ListBindings {
    const ACTION: &'static str = "ListBindings";
    type Query = ListBindingsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListBindingsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListBindingsOps {
    /// List binding relationships between datasets and OSS buckets.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listbindings>
    fn list_bindings(&self, query: ListBindingsQuery) -> impl Future<Output = Result<ListBindingsResponse>>;
}

impl ListBindingsOps for Client {
    async fn list_bindings(&self, query: ListBindingsQuery) -> Result<ListBindingsResponse> {
        self.request(ListBindings { query }).await
    }
}

pub trait BindingOperations: CreateBindingOps + GetBindingOps + DeleteBindingOps + ListBindingsOps {}
impl<T> BindingOperations for T where T: CreateBindingOps + GetBindingOps + DeleteBindingOps + ListBindingsOps {}
