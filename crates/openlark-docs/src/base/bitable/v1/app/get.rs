//! 获取多维表格元数据
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAppRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAppResponse {
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

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetAppBuilder {
    api_req: ApiRequest<GetAppRequest>,
    app_token: String,
}

impl GetAppBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_app_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}",
            builder.app_token
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
