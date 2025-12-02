//! Base V2 列出自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::ListRolesResponse;
use super::RoleService;

/// 列出自定义角色请求
pub struct ListRolesV2Request {
    api_request: ApiRequest<ListRolesV2Response>,
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
            api_request: ApiRequest::get("/open-apis/base/v2/apps/:app_token/roles"),
            app_token: String::new(),
            page_size: None,
            page_token: None,
            role_type: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面 token
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置角色类型过滤
    pub fn role_type(mut self, role_type: String) -> Self {
        self.role_type = Some(role_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRolesV2Response> {
        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles", self.app_token);

        // 创建新的API请求
        let mut api_request: ApiRequest<ListRolesV2Response> =
            ApiRequest::get(&format!("https://open.feishu.cn{}", path));

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
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesV2Response {
    /// 角色列表
    pub data: ListRolesResponse,
    pub success: bool,
}

impl ApiResponseTrait for ListRolesV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自定义角色Builder
pub struct ListRolesV2Builder {
    request: ListRolesV2Request,
}

impl ListRolesV2Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListRolesV2Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置页面 token
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置角色类型过滤
    pub fn role_type(mut self, role_type: String) -> Self {
        self.request = self.request.role_type(role_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListRolesV2Request {
        self.request
    }
}

impl RoleService {
    /// 创建列出角色请求构建器
    pub fn list_roles_v2_builder(&self) -> ListRolesV2Builder {
        ListRolesV2Builder::new(self.config.clone())
    }

    /// 创建列出角色请求
    pub fn list_roles_v2(
        &self,
        app_token: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        role_type: Option<String>,
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
