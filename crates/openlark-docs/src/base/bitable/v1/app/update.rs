//! 更新多维表格元数据
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAppResponse {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct App {
    pub app_token: String,
    pub name: String,
    pub revision: i32,
    pub is_advanced: bool,
    pub time_zone: String,
}

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateAppBuilder {
    api_req: ApiRequest<UpdateAppRequest>,
    app_token: String,
}

impl UpdateAppBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_app_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}",
            builder.app_token
        );
        builder.api_req.body = Some(UpdateAppRequest::default());
        builder
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = Some(name.to_string());
        }
        self
    }

    pub fn is_advanced(mut self, is_advanced: bool) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.is_advanced = Some(is_advanced);
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
