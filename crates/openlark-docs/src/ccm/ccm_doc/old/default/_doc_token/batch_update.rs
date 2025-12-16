//! 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocRequest {
    pub revision: i32,
    pub requests: Vec<DocRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DocRequest {
    // This part involves complex JSON structure for doc edits.
    // Simplifying for now as raw JSON or specific structs if needed.
    // Using simple placeholder or recursive struct?
    // Let's use serde_json::Value for flexibility in this complex update
    #[serde(flatten)]
    pub request: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocResponse {}

impl ApiResponseTrait for BatchUpdateDocResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchUpdateDocBuilder {
    api_req: ApiRequest<BatchUpdateDocRequest>,
    doc_token: String,
}

impl BatchUpdateDocBuilder {
    pub fn new(doc_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_doc_batch_update".to_string();
        builder.api_req.method = "POST".to_string();
        builder.doc_token = doc_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/doc/v2/{}/batch_update",
            builder.doc_token
        );
        builder.api_req.body = Some(BatchUpdateDocRequest::default());
        builder
    }

    pub fn revision(mut self, revision: i32) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.revision = revision;
        }
        self
    }

    pub fn requests(mut self, requests: Vec<DocRequest>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.requests = requests;
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
