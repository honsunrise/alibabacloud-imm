//! Contextual operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualAnswerQuery {
    pub project_name: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualAnswerBody {
    pub messages: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualAnswerResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub answer: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// AI assistant Q&A API that generates contextual answers in conversations.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-contextualanswer>
pub struct ContextualAnswer {
    pub query: ContextualAnswerQuery,
    pub body: ContextualAnswerBody,
}

impl Ops for ContextualAnswer {
    const ACTION: &'static str = "ContextualAnswer";
    type Query = ContextualAnswerQuery;
    type Body = ContextualAnswerBody;
    type Response = BodyResponseProcessor<ContextualAnswerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait ContextualAnswerOps {
    /// AI assistant Q&A API that generates contextual answers in conversations.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-contextualanswer>
    fn contextual_answer(
        &self,
        query: ContextualAnswerQuery,
        body: ContextualAnswerBody,
    ) -> impl Future<Output = Result<ContextualAnswerResponse>>;
}

impl ContextualAnswerOps for Client {
    async fn contextual_answer(
        &self,
        query: ContextualAnswerQuery,
        body: ContextualAnswerBody,
    ) -> Result<ContextualAnswerResponse> {
        self.request(ContextualAnswer { query, body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualRetrievalQuery {
    pub project_name: String,
    pub dataset_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualRetrievalBody {
    pub messages: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_cluster_ids: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextualRetrievalResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub results: Vec<serde_json::Value>,
}

/// Retrieval API for multi-turn conversations that searches semantically similar documents based on context.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-contextualretrieval>
pub struct ContextualRetrieval {
    pub query: ContextualRetrievalQuery,
    pub body: ContextualRetrievalBody,
}

impl Ops for ContextualRetrieval {
    const ACTION: &'static str = "ContextualRetrieval";
    type Query = ContextualRetrievalQuery;
    type Body = ContextualRetrievalBody;
    type Response = BodyResponseProcessor<ContextualRetrievalResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, self.body)
    }
}

pub trait ContextualRetrievalOps {
    /// Retrieval API for multi-turn conversations that searches semantically similar documents based on context.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-contextualretrieval>
    fn contextual_retrieval(
        &self,
        query: ContextualRetrievalQuery,
        body: ContextualRetrievalBody,
    ) -> impl Future<Output = Result<ContextualRetrievalResponse>>;
}

impl ContextualRetrievalOps for Client {
    async fn contextual_retrieval(
        &self,
        query: ContextualRetrievalQuery,
        body: ContextualRetrievalBody,
    ) -> Result<ContextualRetrievalResponse> {
        self.request(ContextualRetrieval { query, body }).await
    }
}

pub trait ContextualOperations: ContextualAnswerOps + ContextualRetrievalOps {}
impl<T> ContextualOperations for T where T: ContextualAnswerOps + ContextualRetrievalOps {}
