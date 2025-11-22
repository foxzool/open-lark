//! Base V2 更新自定义角色API

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

use super::{UpdateRoleRequest, RoleService};

/// 更新自定义角色请求
#[derive(Clone)]
pub struct UpdateRoleV2Request {
    api_request: ApiRequest,
    app_token: String,
    role_id: String,
    /// 角色名称
    name: Option<String>,
    /// 角色描述
    description: Option<String>,
    /// 角色权限列表
    permissions: Option<Vec<String>>,
    /// 角色类型
    role_type: Option<String>,
}

/// 更新自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleV2Response {
    pub data: super::models::Role,
    pub success: bool,
}

impl UpdateRoleV2Request {
    /// 创建更新自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            role_id: String::new(),
            name: None,
            description: None,
            permissions: None,
            role_type: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = Some(permissions);
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: String) -> Self {
        self.role_type = Some(role_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateRoleV2Response> {
        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles/{}", self.app_token, self.role_id);

        // 构建请求体
        let request_body = super::models::UpdateRoleRequest {
            role_id: self.role_id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            permissions: self.permissions.clone(),
            role_type: self.role_type.clone(),
            extra: None,
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(openlark_core::error::SDKError::ValidationError(format!("更新角色请求验证失败: {}", e)));
        }

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::PUT)
            .path(&path)
            .body(serde_json::to_vec(&request_body)?)
            .execute::<UpdateRoleV2Response>()
            .await?;

        Ok(response)
    }
}

/// 更新自定义角色Builder
#[derive(Clone)]
pub struct UpdateRoleV2Builder {
    request: UpdateRoleV2Request,
}

impl UpdateRoleV2Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateRoleV2Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
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
    pub fn build(self) -> UpdateRoleV2Request {
        self.request
    }
}

impl RoleService {
    /// 创建更新自定义角色请求构建器
    pub fn update_role_v2_builder(&self, config: Config) -> UpdateRoleV2Builder {
        UpdateRoleV2Builder::new(config)
    }

    /// 创建更新自定义角色请求
    pub fn update_role_v2(&self, app_token: String, role_id: String, name: Option<String>, description: Option<String>, permissions: Option<Vec<String>>, role_type: Option<String>) -> UpdateRoleV2Request {
        UpdateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id)
            .name(name.unwrap_or_default())
            .description(description.unwrap_or_default())
            .permissions(permissions.unwrap_or_default())
            .role_type(role_type.unwrap_or_default())
    }
}