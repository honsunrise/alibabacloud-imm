//! Region operations.

use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Request};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRegionsQuery {
    pub accept_language: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRegionsResponse {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub regions: Vec<serde_json::Value>,
}

/// List regions that support the IMM service.
///
/// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listregions>
pub struct ListRegions {
    pub query: ListRegionsQuery,
}

impl Ops for ListRegions {
    const ACTION: &'static str = "ListRegions";
    type Query = ListRegionsQuery;
    type Body = ();
    type Response = BodyResponseProcessor<ListRegionsResponse>;

    fn into_parts(self) -> (Self::Query, Self::Body) {
        (self.query, ())
    }
}

pub trait ListRegionsOps {
    /// List regions that support the IMM service.
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-listregions>
    fn list_regions(&self, query: ListRegionsQuery) -> impl Future<Output = Result<ListRegionsResponse>>;
}

impl ListRegionsOps for Client {
    async fn list_regions(&self, query: ListRegionsQuery) -> Result<ListRegionsResponse> {
        self.request(ListRegions { query }).await
    }
}
