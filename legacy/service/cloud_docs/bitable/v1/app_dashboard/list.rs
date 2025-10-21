use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::bitable::v1::app_dashboard::Dashboard,
};

/// 列出仪表盘请求
#[derive(Debug, Serialize, Default)]
pub struct ListDashboardRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListDashboardRequest {
    pub fn builder() -> ListDashboardRequestBuilder {
        ListDashboardRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListDashboardRequestBuilder {
    request: ListDashboardRequest,
}

impl ListDashboardRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn build(mut self) -> ListDashboardRequest {
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        self.request
    }
}

// 应用ExecutableBuilder trait到ListDashboardRequestBuilder
crate::impl_executable_builder_owned!(
    ListDashboardRequestBuilder,
    DashboardService,
    ListDashboardRequest,
    BaseResponse<ListDashboardResponse>,
    list
);

/// 列出仪表盘响应
#[derive(Debug, Deserialize)]
pub struct ListDashboardResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 仪表盘信息列表
    pub items: Vec<Dashboard>,
}

impl ApiResponseTrait for ListDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 仪表盘服务
pub struct DashboardService {
    pub config: Config,
}

impl DashboardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出仪表盘
    pub async fn list(
        &self,
        request: ListDashboardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListDashboardResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_DASHBOARDS.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 列出仪表盘 (向后兼容的函数)
pub async fn list_dashboard(
    request: ListDashboardRequest,
    config: Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListDashboardResponse>> {
    let service = DashboardService::new(config);
    service.list(request, option).await
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dashboard_request_builder() {
        let request = ListDashboardRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(20)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
    }
}
