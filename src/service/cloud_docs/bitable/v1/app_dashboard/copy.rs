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
    impl_executable_builder_config,
};

/// 复制仪表盘请求
#[derive(Debug, Serialize, Default)]
pub struct CopyDashboardRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 仪表盘ID
    #[serde(skip)]
    block_id: String,
    /// 复制后的仪表盘名称
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl CopyDashboardRequest {
    pub fn builder() -> CopyDashboardRequestBuilder {
        CopyDashboardRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, block_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            block_id: block_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest,
}

impl CopyDashboardRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 仪表盘ID
    pub fn block_id(mut self, block_id: impl ToString) -> Self {
        self.request.block_id = block_id.to_string();
        self
    }

    /// 复制后的仪表盘名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    pub fn build(mut self) -> CopyDashboardRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_config!(
    CopyDashboardRequestBuilder,
    CopyDashboardRequest,
    BaseResponse<CopyDashboardResponse>,
    copy_dashboard
);

/// 仪表盘信息
#[derive(Debug, Deserialize)]
pub struct Dashboard {
    /// 仪表盘ID
    pub block_id: String,
    /// 仪表盘名称
    pub name: String,
}

/// 复制仪表盘响应
#[derive(Debug, Deserialize)]
pub struct CopyDashboardResponse {
    /// 复制后的仪表盘信息
    pub dashboard: Dashboard,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制仪表盘
pub async fn copy_dashboard(
    request: CopyDashboardRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CopyDashboardResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_DASHBOARD_COPY
        .replace("{app_token}", &request.app_token)
        .replace("{block_id}", &request.block_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dashboard_request_builder() {
        let request = CopyDashboardRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .block_id("blkxxxxxx")
            .name("复制的仪表盘")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.block_id, "blkxxxxxx");
        assert_eq!(request.name, Some("复制的仪表盘".to_string()));
    }
}
