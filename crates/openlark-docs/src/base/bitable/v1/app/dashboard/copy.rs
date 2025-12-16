//! 该接口用于根据现有仪表盘复制出新的仪表盘
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/copy

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyDashboardRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyDashboardResponse {
    pub block_id: String,
    pub name: String,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CopyDashboardBuilder {
    api_req: ApiRequest<CopyDashboardRequest>,
    app_token: String,
    block_id: String,
}

impl CopyDashboardBuilder {
    pub fn new(app_token: impl ToString, block_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_dashboard_copy".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.block_id = block_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/dashboards/{}/copy",
            builder.app_token, builder.block_id
        );
        builder.api_req.body = Some(CopyDashboardRequest::default());
        builder
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = name.to_string();
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
