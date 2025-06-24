use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder,
    service::bitable::v1::app_role::AppRole,
};

/// 列出自定义角色请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListAppRoleRequest {
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

impl ListAppRoleRequest {
    pub fn builder() -> ListAppRoleRequestBuilder {
        ListAppRoleRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListAppRoleRequestBuilder {
    request: ListAppRoleRequest,
}

impl ListAppRoleRequestBuilder {
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

    pub fn build(mut self) -> ListAppRoleRequest {
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token".to_string(), page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        self.request
    }

}

// 应用ExecutableBuilder trait到ListAppRoleRequestBuilder
impl_executable_builder!(
    ListAppRoleRequestBuilder,
    super::AppRoleService,
    ListAppRoleRequest,
    BaseResponse<ListAppRoleResponse>,
    list
);

/// 列出自定义角色响应
#[derive(Debug, Deserialize)]
pub struct ListAppRoleResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 自定义角色信息列表
    pub items: Vec<AppRole>,
}

impl ApiResponseTrait for ListAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自定义角色
pub async fn list_app_roles(
    request: ListAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListAppRoleResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "/open-apis/bitable/v1/apps/{app_token}/roles",
        app_token = request.app_token
    );
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_app_role_request_builder() {
        let request = ListAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(20)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
    }
}
