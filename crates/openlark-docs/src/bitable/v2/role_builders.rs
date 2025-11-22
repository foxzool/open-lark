//! V2 角色管理 API 构建器
//!
//! 提供流畅的构建器接口用于 V2 角色管理 API：
//! - 创建角色构建器
//! - 查询角色构建器
//! - 更新角色构建器

use super::role_management::*;
use openlark_core::config::Config;

/// 创建角色 V2 构建器
#[derive(Debug, Clone)]
pub struct CreateRoleV2RequestBuilder {
    request: CreateRoleV2Request,
    config: Config,
    app_token: String,
}

impl CreateRoleV2RequestBuilder {
    /// 创建新的角色构建器
    pub fn new(config: Config, app_token: String, name: String) -> Self {
        Self {
            request: CreateRoleV2Request {
                name,
                description: None,
                permissions: Vec::new(),
            },
            config,
            app_token,
        }
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.request.description = Some(description);
        self
    }

    /// 添加权限
    pub fn permission(mut self, permission: String) -> Self {
        self.request.permissions.push(permission);
        self
    }

    /// 设置权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request.permissions = permissions;
        self
    }

    /// 执行创建角色请求
    pub async fn execute(self) -> SDKResult<CreateRoleV2Response> {
        let service = RoleManagementV2Service::new(self.config);
        service.create_role_v2(&self.app_token, &self.request).await
    }
}

/// 列出角色 V2 构建器
#[derive(Debug, Clone)]
pub struct ListRolesV2RequestBuilder {
    config: Config,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRolesV2RequestBuilder {
    /// 创建新的角色列表构建器
    pub fn new(config: Config, app_token: String) -> Self {
        Self {
            config,
            app_token,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 设置分页 token
    pub fn page_token(mut self, token: String) -> Self {
        self.page_token = Some(token);
        self
    }

    /// 执行列出角色请求
    pub async fn execute(self) -> SDKResult<ListRolesV2Response> {
        let service = RoleManagementV2Service::new(self.config);
        service
            .list_roles_v2(&self.app_token, self.page_size, self.page_token.as_deref())
            .await
    }
}

/// 更新角色 V2 构建器
#[derive(Debug, Clone)]
pub struct UpdateRoleV2RequestBuilder {
    request: UpdateRoleV2Request,
    config: Config,
    app_token: String,
    role_id: String,
}

impl UpdateRoleV2RequestBuilder {
    /// 创建新的角色更新构建器
    pub fn new(config: Config, app_token: String, role_id: String) -> Self {
        Self {
            request: UpdateRoleV2Request {
                name: None,
                description: None,
                permissions: Vec::new(),
            },
            config,
            app_token,
            role_id,
        }
    }

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.request.name = Some(name);
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.request.description = Some(description);
        self
    }

    /// 添加权限
    pub fn permission(mut self, permission: String) -> Self {
        self.request.permissions.push(permission);
        self
    }

    /// 设置权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request.permissions = permissions;
        self
    }

    /// 执行更新角色请求
    pub async fn execute(self) -> SDKResult<UpdateRoleV2Response> {
        let service = RoleManagementV2Service::new(self.config);
        service
            .update_role_v2(&self.app_token, &self.role_id, &self.request)
            .await
    }
}
