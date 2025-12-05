//! Base V2 列出自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::ListRolesResponse;
use super::RoleService;

/// 列出自定义角色请求
pub struct ListRolesV2Request {
    app_token: String,
    /// 页面大小
    page_size: Option<i32>,
    /// 页面 token
    page_token: Option<String>,
    /// 角色类型过滤
    role_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRolesV2Request {
    /// 创建列出角色请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            page_size: None,
            page_token: None,
            role_type: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面 token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置角色类型过滤
    pub fn role_type(mut self, role_type: impl Into<String>) -> Self {
        self.role_type = Some(role_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRolesV2Response> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles", self.app_token);

        // 创建API请求
        let mut api_request: ApiRequest<ListRolesV2Response> = ApiRequest::get(&path);

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(ref role_type) = self.role_type {
            api_request = api_request.query("role_type", role_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesV2Response {
    /// 角色列表
    pub data: ListRolesResponse,
}

impl ApiResponseTrait for ListRolesV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RoleService {
    /// 创建列出角色请求
    pub fn list_roles_v2_builder(
        &self,
        app_token: impl Into<String>,
    ) -> ListRolesV2Request {
        ListRolesV2Request::new(self.config.clone()).app_token(app_token)
    }

    /// 创建列出角色请求（带完整参数）
    pub fn list_roles_v2(
        &self,
        app_token: impl Into<String>,
        page_size: Option<i32>,
        page_token: Option<impl Into<String>>,
        role_type: Option<impl Into<String>>,
    ) -> ListRolesV2Request {
        let mut request = ListRolesV2Request::new(self.config.clone()).app_token(app_token);

        if let Some(page_size) = page_size {
            request = request.page_size(page_size);
        }

        if let Some(page_token) = page_token {
            request = request.page_token(page_token);
        }

        if let Some(role_type) = role_type {
            request = request.role_type(role_type);
        }

        request
    }
}
