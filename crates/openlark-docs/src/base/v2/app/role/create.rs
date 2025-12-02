//! Base V2 创建自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{RoleService, models::{RoleResponse as Role, CreateRoleRequest}};

/// 新增自定义角色请求
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
    /// 配置信息
    config: Config,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleV2Response {
    pub data: Role,
    pub success: bool,
}

impl ApiResponseTrait for CreateRoleV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoleV2Request {
    /// 创建新增自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/base/v2/apps/:app_token/roles"),
            app_token: String::new(),
            name: String::new(),
            description: None,
            permissions: Vec::new(),
            role_type: None,
            config,
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
    pub async fn execute(self) -> SDKResult<CreateRoleV2Response> {
        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles", self.app_token);

        // 构建请求体 - 修正字段名称映射
        let request_body = CreateRoleRequest {
            role_name: self.name.clone(),
            table_roles: None, // 根据实际API需求设置
        };

        // 创建新的API请求
        let api_request: ApiRequest<CreateRoleV2Response> = ApiRequest::post(&format!("https://open.feishu.cn{}", path))
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 创建自定义角色Builder
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
        let mut request = CreateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .name(name)
            .permissions(permissions);

        if let Some(desc) = description {
            request = request.description(desc);
        }

        if let Some(rtype) = role_type {
            request = request.role_type(rtype);
        }

        request
    }
}