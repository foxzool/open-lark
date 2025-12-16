//! 在指定目录下创建多维表格
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateAppResponse {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct App {
    pub app_token: String,
    pub name: String,
    pub folder_token: String,
    pub url: String,
    pub default_table_id: String,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateAppBuilder {
    api_req: ApiRequest<CreateAppRequest>,
}

impl CreateAppBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_app_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/bitable/v1/apps".to_string();
        builder.api_req.body = Some(CreateAppRequest::default());
        builder
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = Some(name.to_string());
        }
        self
    }

    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.folder_token = Some(folder_token.to_string());
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
