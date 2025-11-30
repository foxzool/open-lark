//! Base V2 创建自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::RoleService;

/// 新增自定义角色请求
#[derive(Clone)]
pub struct CreateRoleV2Request {
    api_request: ApiRequest<CreateRoleV2Response>,
    app_token: String,
    /// 角色名称
    name: String,
    /// 角色描述
    description: Option<String>,
    /// 角色权限列表
    permissions: Vec<String>,
    /// 角色类型
    role_type: Option<String>,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleV2Response {
    pub data: Role,
    pub success: bool,
}

impl CreateRoleV2Request {
    /// 创建新增自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Post)
                .api_path("/open-apis/base/v2/apps/:app_token/roles".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            name: String::new(),
            description: None,
            permissions: Vec::new(),
            role_type: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = permissions;
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: String) -> Self {
        self.role_type = Some(role_type);
        self
    }

    /// 执行请求
    pub async fn execute(mut self) -> SDKResult<CreateRoleV2Response> {
        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles", self.app_token);

        // 更新API路径
        self.api_request = self.api_request.api_path(path);

        // 构建请求体
        let request_body = serde_json::json!({
            "name": self.name,
            "description": self.description,
            "permissions": self.permissions,
            "role_type": self.role_type
        });

        // 设置请求体
        self.api_request = self.api_request.body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 创建自定义角色Builder
#[derive(Clone)]
pub struct CreateRoleV2Builder {
    request: CreateRoleV2Request,
}

impl CreateRoleV2Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateRoleV2Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request = self.request.permissions(permissions);
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: String) -> Self {
        self.request = self.request.role_type(role_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateRoleV2Request {
        self.request
    }
}

impl RoleService {
    /// 创建新增自定义角色请求构建器
    pub fn create_role_v2_builder(&self) -> CreateRoleV2Builder {
        CreateRoleV2Builder::new(self.config.clone())
    }

    /// 创建新增自定义角色请求
    pub fn create_role_v2(&self, app_token: String, name: String, description: Option<String>, permissions: Vec<String>, role_type: Option<String>) -> CreateRoleV2Request {
        CreateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .name(name)
            .description(description.unwrap_or_default())
            .permissions(permissions)
            .role_type(role_type.unwrap_or_default())
    }
}