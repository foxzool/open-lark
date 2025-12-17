/// Bitable 复制仪表盘
///
/// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/copy
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/copy
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 复制仪表盘请求
#[derive(Debug, Clone)]
pub struct CopyDashboardRequest {
    config: Config,
    app_token: String,
    block_id: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    name: Option<String>,
}

impl CopyDashboardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            block_id: String::new(),
            user_id_type: None,
            client_token: None,
            name: None,
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = block_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 新仪表盘名称（如果文档支持）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CopyDashboardResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.block_id, "block_id 不能为空");

        let api_endpoint = BitableApiV1::DashboardCopy(self.app_token, self.block_id);
        let mut api_request: ApiRequest<CopyDashboardResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(
                &CopyDashboardRequestBody { name: self.name },
            )?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 复制仪表盘 Builder
pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest,
}

impl CopyDashboardRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: CopyDashboardRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.request = self.request.block_id(block_id);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request = self.request.name(name);
        self
    }

    pub fn build(self) -> CopyDashboardRequest {
        self.request
    }
}

#[derive(Debug, Serialize)]
struct CopyDashboardRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// 复制仪表盘响应（不同版本可能返回 `block_id` 或 `dashboard` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyDashboardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Dashboard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
