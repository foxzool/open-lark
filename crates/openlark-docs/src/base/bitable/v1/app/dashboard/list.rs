/// Bitable 列出仪表盘
///
/// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/list
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 列出仪表盘请求
#[derive(Debug, Clone)]
pub struct ListDashboardsRequest {
    config: Config,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListDashboardsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListDashboardsResponse> {
        validate_required!(self.app_token, "app_token 不能为空");

        let api_endpoint = BitableApiV1::DashboardList(self.app_token);
        let mut api_request: ApiRequest<ListDashboardsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        api_request = api_request.query_opt(
            "page_size",
            self.page_size.map(|v| v.min(100).to_string()),
        );
        api_request = api_request.query_opt("page_token", self.page_token);
        api_request = api_request.query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
    }
}

/// 列出仪表盘 Builder
pub struct ListDashboardsRequestBuilder {
    request: ListDashboardsRequest,
}

impl ListDashboardsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: ListDashboardsRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn build(self) -> ListDashboardsRequest {
        self.request
    }
}

/// 列出仪表盘响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDashboardsResponse {
    pub dashboards: Vec<Dashboard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub block_id: String,
    pub name: String,
}

impl ApiResponseTrait for ListDashboardsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

