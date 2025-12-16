//! 创建并初始化文档。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDocRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folderToken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDocResponse {
    pub objToken: String,
    pub url: String,
}

impl ApiResponseTrait for CreateDocResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateDocBuilder {
    api_req: ApiRequest<CreateDocRequest>,
}

impl CreateDocBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_doc_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/doc/v2/create".to_string();
        builder.api_req.body = Some(CreateDocRequest::default());
        builder
    }

    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.folderToken = Some(folder_token.to_string());
        }
        self
    }

    pub fn content(mut self, content: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.content = Some(content.to_string());
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
