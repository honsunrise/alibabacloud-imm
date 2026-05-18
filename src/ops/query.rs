//! Query operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleQueryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_total_hits: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleQueryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
    #[serde(default)]
    pub aggregations: Vec<serde_json::Value>,
    #[serde(default)]
    pub total_hits: Option<i64>,
}

/// Query and aggregate file metadata in a dataset using logical expressions.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-simplequery>
pub struct SimpleQuery {
    pub query: SimpleQueryQuery,
}

impl Ops for SimpleQuery {
    const ACTION: &'static str = "SimpleQuery";
    type Query = SimpleQueryQuery;
    type Body = ();
    type Response = BodyResponseProcessor<SimpleQueryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait SimpleQueryOps {
    /// Query and aggregate file metadata in a dataset using logical expressions.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-simplequery>
    fn simple_query(&self, query: SimpleQueryQuery) -> impl Future<Output = Result<SimpleQueryResponse>>;
}

impl SimpleQueryOps for Client {
    async fn simple_query(&self, query: SimpleQueryQuery) -> Result<SimpleQueryResponse> {
        self.request(SimpleQuery { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SemanticQueryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_types: Option<serde_json::Value>,
    #[serde(rename = "SourceURI", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SemanticQueryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
}

/// Query file metadata in a dataset using natural language semantics.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-semanticquery>
pub struct SemanticQuery {
    pub query: SemanticQueryQuery,
}

impl Ops for SemanticQuery {
    const ACTION: &'static str = "SemanticQuery";
    type Query = SemanticQueryQuery;
    type Body = ();
    type Response = BodyResponseProcessor<SemanticQueryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait SemanticQueryOps {
    /// Query file metadata in a dataset using natural language semantics.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-semanticquery>
    fn semantic_query(
        &self,
        query: SemanticQueryQuery,
    ) -> impl Future<Output = Result<SemanticQueryResponse>>;
}

impl SemanticQueryOps for Client {
    async fn semantic_query(&self, query: SemanticQueryQuery) -> Result<SemanticQueryResponse> {
        self.request(SemanticQuery { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuzzyQueryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    pub project_name: String,
    pub dataset_name: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuzzyQueryResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub files: Vec<serde_json::Value>,
    #[serde(default)]
    pub total_hits: Option<i64>,
}

/// Fuzzy query that matches file metadata fields against a specified string.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-fuzzyquery>
pub struct FuzzyQuery {
    pub query: FuzzyQueryQuery,
}

impl Ops for FuzzyQuery {
    const ACTION: &'static str = "FuzzyQuery";
    type Query = FuzzyQueryQuery;
    type Body = ();
    type Response = BodyResponseProcessor<FuzzyQueryResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait FuzzyQueryOps {
    /// Fuzzy query that matches file metadata fields against a specified string.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-fuzzyquery>
    fn fuzzy_query(&self, query: FuzzyQueryQuery) -> impl Future<Output = Result<FuzzyQueryResponse>>;
}

impl FuzzyQueryOps for Client {
    async fn fuzzy_query(&self, query: FuzzyQueryQuery) -> Result<FuzzyQueryResponse> {
        self.request(FuzzyQuery { query }).await
    }
}

pub trait QueryOperations: SimpleQueryOps + SemanticQueryOps + FuzzyQueryOps {}
impl<T> QueryOperations for T where T: SimpleQueryOps + SemanticQueryOps + FuzzyQueryOps {}
