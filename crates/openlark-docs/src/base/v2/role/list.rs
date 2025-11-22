//! Base V2 列出自定义角色API

#![allow(unused_variables, unused_imports, dead_code, non_snake_case)]
#![allow(clippy::too_many_arguments)]

use openlark_core::{
    api::ApiRequest,
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{ListRolesRequest, ListRolesResponse, RoleService};

/// 列出自定义角色请求
#[derive(Clone)]
pub struct ListRolesV2Request {
    api_request: ApiRequest,
    app_token: String,
    /// 页面大小
    page_size: Option<i32>,
    /// 页面 token
    page_token: Option<String>,
    /// 角色类型过滤
    role_type: Option<String>,
}

impl ListRolesV2Request {
    /// 创建列出角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            page_size: None,
            page_token: None,
            role_type: None,
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

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(ref page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(ref page_token) = self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if let Some(ref role_type) = self.role_type {
            query_params.push(format!("role_type={}", role_type));
        }

        // 构建查询字符串
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        // 构建完整路径
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{}{}", path, query_string)
        };

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::GET)
            .path(&full_path)
            .execute::<ListRolesV2Response>()
            .await?;

        Ok(response)
    }
}

/// 列出自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesV2Response {
    /// 角色列表
    pub data: super::models::ListRolesResponse,
    pub success: bool,
}

/// 列出自定义角色Builder
#[derive(Clone)]
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
    pub fn list_roles_v2_builder(&self, config: Config) -> ListRolesV2Builder {
        ListRolesV2Builder::new(config)
    }

    /// 创建列出角色请求
    pub fn list_roles_v2(&self, app_token: String, page_size: Option<i32>, page_token: Option<String>, role_type: Option<String>) -> ListRolesV2Request {
        let mut request = ListRolesV2Request::new(self.config.clone())
            .app_token(app_token);

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