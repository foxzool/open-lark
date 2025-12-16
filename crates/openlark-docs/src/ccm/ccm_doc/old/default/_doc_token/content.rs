//! 获取结构化的文档内容。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocContentRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocContentResponse {
    pub content: String,
    pub revision: i32,
}

impl ApiResponseTrait for GetDocContentResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetDocContentBuilder {
    api_req: ApiRequest<GetDocContentRequest>,
    doc_token: String,
}

impl GetDocContentBuilder {
    pub fn new(doc_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_doc_content_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.doc_token = doc_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/doc/v2/{}/content",
            builder.doc_token
        );
        builder.api_req.body = None;
        builder
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
