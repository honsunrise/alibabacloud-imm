//! Trigger operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTriggerBody {
    pub project_name: String,
    pub input: String,
    pub actions: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub service_role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTriggerResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub id: Option<String>,
}

/// Create a trigger that executes IMM data processing on OSS events.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createtrigger>
pub struct CreateTrigger {
    pub body: CreateTriggerBody,
}

impl Ops for CreateTrigger {
    const ACTION: &'static str = "CreateTrigger";
    type Query = ();
    type Body = CreateTriggerBody;
    type Response = BodyResponseProcessor<CreateTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait CreateTriggerOps {
    /// Create a trigger that executes IMM data processing on OSS events.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-createtrigger>
    fn create_trigger(&self, body: CreateTriggerBody) -> impl Future<Output = Result<CreateTriggerResponse>>;
}

impl CreateTriggerOps for Client {
    async fn create_trigger(&self, body: CreateTriggerBody) -> Result<CreateTriggerResponse> {
        self.request(CreateTrigger { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTriggerQuery {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTriggerResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub trigger: Option<String>,
}

/// Get trigger information.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-gettrigger>
pub struct GetTrigger {
    pub query: GetTriggerQuery,
}

impl Ops for GetTrigger {
    const ACTION: &'static str = "GetTrigger";
    type Query = GetTriggerQuery;
    type Body = ();
    type Response = BodyResponseProcessor<GetTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait GetTriggerOps {
    /// Get trigger information.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-gettrigger>
    fn get_trigger(&self, query: GetTriggerQuery) -> impl Future<Output = Result<GetTriggerResponse>>;
}

impl GetTriggerOps for Client {
    async fn get_trigger(&self, query: GetTriggerQuery) -> Result<GetTriggerResponse> {
        self.request(GetTrigger { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTriggersQuery {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTriggersResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub triggers: Vec<serde_json::Value>,
    #[serde(default)]
    pub next_token: Option<String>,
}

/// List triggers with sorting and filtering support.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listtriggers>
pub struct ListTriggers {
    pub query: ListTriggersQuery,
}

impl Ops for ListTriggers {
    const ACTION: &'static str = "ListTriggers";
    type Query = ListTriggersQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListTriggersResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListTriggersOps {
    /// List triggers with sorting and filtering support.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listtriggers>
    fn list_triggers(&self, query: ListTriggersQuery) -> impl Future<Output = Result<ListTriggersResponse>>;
}

impl ListTriggersOps for Client {
    async fn list_triggers(&self, query: ListTriggersQuery) -> Result<ListTriggersResponse> {
        self.request(ListTriggers { query }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTriggerBody {
    pub project_name: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTriggerResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Update trigger configuration.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatetrigger>
pub struct UpdateTrigger {
    pub body: UpdateTriggerBody,
}

impl Ops for UpdateTrigger {
    const ACTION: &'static str = "UpdateTrigger";
    type Query = ();
    type Body = UpdateTriggerBody;
    type Response = BodyResponseProcessor<UpdateTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait UpdateTriggerOps {
    /// Update trigger configuration.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-updatetrigger>
    fn update_trigger(&self, body: UpdateTriggerBody) -> impl Future<Output = Result<UpdateTriggerResponse>>;
}

impl UpdateTriggerOps for Client {
    async fn update_trigger(&self, body: UpdateTriggerBody) -> Result<UpdateTriggerResponse> {
        self.request(UpdateTrigger { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTriggerBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTriggerResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Delete a trigger.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletetrigger>
pub struct DeleteTrigger {
    pub body: DeleteTriggerBody,
}

impl Ops for DeleteTrigger {
    const ACTION: &'static str = "DeleteTrigger";
    type Query = ();
    type Body = DeleteTriggerBody;
    type Response = BodyResponseProcessor<DeleteTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait DeleteTriggerOps {
    /// Delete a trigger.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-deletetrigger>
    fn delete_trigger(&self, body: DeleteTriggerBody) -> impl Future<Output = Result<DeleteTriggerResponse>>;
}

impl DeleteTriggerOps for Client {
    async fn delete_trigger(&self, body: DeleteTriggerBody) -> Result<DeleteTriggerResponse> {
        self.request(DeleteTrigger { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspendTriggerBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspendTriggerResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Suspend a running trigger.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-suspendtrigger>
pub struct SuspendTrigger {
    pub body: SuspendTriggerBody,
}

impl Ops for SuspendTrigger {
    const ACTION: &'static str = "SuspendTrigger";
    type Query = ();
    type Body = SuspendTriggerBody;
    type Response = BodyResponseProcessor<SuspendTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait SuspendTriggerOps {
    /// Suspend a running trigger.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-suspendtrigger>
    fn suspend_trigger(
        &self,
        body: SuspendTriggerBody,
    ) -> impl Future<Output = Result<SuspendTriggerResponse>>;
}

impl SuspendTriggerOps for Client {
    async fn suspend_trigger(&self, body: SuspendTriggerBody) -> Result<SuspendTriggerResponse> {
        self.request(SuspendTrigger { body }).await
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeTriggerBody {
    pub project_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResumeTriggerResponse {
    #[serde(default)]
    pub request_id: String,
}

/// Resume a suspended or failed trigger.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-resumetrigger>
pub struct ResumeTrigger {
    pub body: ResumeTriggerBody,
}

impl Ops for ResumeTrigger {
    const ACTION: &'static str = "ResumeTrigger";
    type Query = ();
    type Body = ResumeTriggerBody;
    type Response = BodyResponseProcessor<ResumeTriggerResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        ((), self.body)
    }
}

pub trait ResumeTriggerOps {
    /// Resume a suspended or failed trigger.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-resumetrigger>
    fn resume_trigger(&self, body: ResumeTriggerBody) -> impl Future<Output = Result<ResumeTriggerResponse>>;
}

impl ResumeTriggerOps for Client {
    async fn resume_trigger(&self, body: ResumeTriggerBody) -> Result<ResumeTriggerResponse> {
        self.request(ResumeTrigger { body }).await
    }
}

pub trait TriggerOperations:
    CreateTriggerOps
    + GetTriggerOps
    + ListTriggersOps
    + UpdateTriggerOps
    + DeleteTriggerOps
    + SuspendTriggerOps
    + ResumeTriggerOps
{
}
impl<T> TriggerOperations for T where
    T: CreateTriggerOps
        + GetTriggerOps
        + ListTriggersOps
        + UpdateTriggerOps
        + DeleteTriggerOps
        + SuspendTriggerOps
        + ResumeTriggerOps
{
}
