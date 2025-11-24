//! 角色管理 V1 API
//!
//! 提供角色的创建、更新、删除和分配功能，支持基于角色的访问控制（RBAC）。

use crate::acs::service::{
    AccessControlService, AssignRoleRequest, AssignRoleResponse, CreateRoleRequest,
    CreateRoleResponse, DeleteRoleRequest, DeleteRoleResponse, ListRolesRequest, ListRolesResponse,
    Role, RoleType, UpdateRoleRequest, UpdateRoleResponse,
};
use crate::error::{ErrorContext, SecurityError, SecurityResult};
use crate::models::{DeviceInfo, DeviceType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

/// 角色管理API v1
pub struct RoleV1API {
    service: Box<dyn AccessControlService>,
}

impl RoleV1API {
    /// 创建新的角色API实例
    pub fn new(service: Box<dyn AccessControlService>) -> Self {
        Self { service }
    }

    /// 创建默认角色API实例
    pub fn default() -> Self {
        Self::new(Box::new(
            crate::acs::service::DefaultAccessControlService::with_default_config(),
        ))
    }
}

impl RoleV1API {
    /// 创建角色
    ///
    /// # 参数
    /// - `request`: 角色创建请求
    ///
    /// # 返回值
    /// 返回角色创建结果，包含角色ID和创建时间
    ///
    /// # 示例
    /// ```rust
    /// let api = RoleV1API::default();
    /// let request = CreateRoleRequestBuilder::new()
    ///     .role_name("文档管理员")
    ///     .description("负责文档管理的角色")
    ///     .add_permission("document:read")
    ///     .add_permission("document:write")
    ///     .creator_id("admin_001")
    ///     .role_type(RoleType::Business)
    ///     .build()?;
    /// let response = api.create_role(request).await?;
    /// println!("角色创建成功: role_id={}", response.role_id);
    /// ```
    pub async fn create_role(
        &self,
        request: CreateRoleRequest,
    ) -> SecurityResult<CreateRoleResponse> {
        let context = ErrorContext::new("create_role")
            .with_user_id(&request.creator_id)
            .with_extra("role_name", &request.role_name);

        info!(
            "开始创建角色: name={}, creator={}",
            request.role_name, request.creator_id
        );

        // 验证请求参数
        validate_create_role_request(&request)?;

        // 调用服务层
        match self.service.create_role(request.clone()).await {
            Ok(response) => {
                info!(
                    "角色创建完成: role_id={}, name={}",
                    response.role_id, response.role_name
                );
                Ok(response)
            }
            Err(e) => {
                error!("角色创建失败: name={}, error={}", request.role_name, e);
                Err(e)
            }
        }
    }

    /// 删除角色
    ///
    /// # 参数
    /// - `request`: 角色删除请求
    ///
    /// # 返回值
    /// 返回角色删除结果
    ///
    /// # 示例
    /// ```rust
    /// let api = RoleV1API::default();
    /// let request = DeleteRoleRequestBuilder::new()
    ///     .role_id("role_001")
    ///     .operator_id("admin_001")
    ///     .build()?;
    /// let response = api.delete_role(request).await?;
    /// if response.deleted {
    ///     println!("角色删除成功");
    /// }
    /// ```
    pub async fn delete_role(
        &self,
        request: DeleteRoleRequest,
    ) -> SecurityResult<DeleteRoleResponse> {
        let context = ErrorContext::new("delete_role")
            .with_user_id(&request.operator_id)
            .with_extra("role_id", &request.role_id);

        info!(
            "开始删除角色: role_id={}, operator={}",
            request.role_id, request.operator_id
        );

        // 验证请求参数
        validate_delete_role_request(&request)?;

        // 调用服务层
        match self.service.delete_role(request.clone()).await {
            Ok(response) => {
                info!("角色删除完成: role_id={}", response.role_id);
                Ok(response)
            }
            Err(e) => {
                error!("角色删除失败: role_id={}, error={}", request.role_id, e);
                Err(e)
            }
        }
    }

    /// 更新角色
    ///
    /// # 参数
    /// - `request`: 角色更新请求
    ///
    /// # 返回值
    /// 返回角色更新结果
    ///
    /// # 示例
    /// ```rust
    /// let api = RoleV1API::default();
    /// let request = UpdateRoleRequestBuilder::new()
    ///     .role_id("role_001")
    ///     .role_name("更新后的角色名称")
    ///     .description("更新后的角色描述")
    ///     .add_permission("document:read")
    ///     .operator_id("admin_001")
    ///     .build()?;
    /// let response = api.update_role(request).await?;
    /// if response.updated {
    ///     println!("角色更新成功");
    /// }
    /// ```
    pub async fn update_role(
        &self,
        request: UpdateRoleRequest,
    ) -> SecurityResult<UpdateRoleResponse> {
        let context = ErrorContext::new("update_role")
            .with_user_id(&request.operator_id)
            .with_extra("role_id", &request.role_id);

        info!(
            "开始更新角色: role_id={}, operator={}",
            request.role_id, request.operator_id
        );

        // 验证请求参数
        validate_update_role_request(&request)?;

        // 调用服务层
        match self.service.update_role(request.clone()).await {
            Ok(response) => {
                info!("角色更新完成: role_id={}", response.role_id);
                Ok(response)
            }
            Err(e) => {
                error!("角色更新失败: role_id={}, error={}", request.role_id, e);
                Err(e)
            }
        }
    }

    /// 获取角色列表
    ///
    /// # 参数
    /// - `request`: 角色列表请求
    ///
    /// # 返回值
    /// 返回角色列表，包含角色详细信息
    ///
    /// # 示例
    /// ```rust
    /// let api = RoleV1API::default();
    /// let request = ListRolesRequestBuilder::new()
    ///     .page_size(20)
    ///     .role_type(RoleType::Business)
    ///     .status("active")
    ///     .build()?;
    /// let response = api.list_roles(request).await?;
    /// println!("角色总数: {}", response.total_count);
    /// for role in response.roles {
    ///     println!("角色: {} ({})", role.role_name, role.role_id);
    /// }
    /// ```
    pub async fn list_roles(&self, request: ListRolesRequest) -> SecurityResult<ListRolesResponse> {
        let context = ErrorContext::new("list_roles");

        info!("开始获取角色列表: page_size={:?}", request.page_size);

        // 验证请求参数
        validate_list_roles_request(&request)?;

        // 调用服务层
        match self.service.list_roles(request.clone()).await {
            Ok(response) => {
                info!("角色列表获取完成: count={}", response.total_count);
                Ok(response)
            }
            Err(e) => {
                error!("角色列表获取失败: error={}", e);
                Err(e)
            }
        }
    }

    /// 分配角色给用户
    ///
    /// # 参数
    /// - `request`: 角色分配请求
    ///
    /// # 返回值
    /// 返回角色分配结果，包含分配ID和分配时间
    ///
    /// # 示例
    /// ```rust
    /// let api = RoleV1API::default();
    /// let request = AssignRoleRequestBuilder::new()
    ///     .operator_id("admin_001")
    ///     .target_user_id("user_001")
    ///     .role_id("role_001")
    ///     .reason("工作需要")
    ///     .build()?;
    /// let response = api.assign_role(request).await?;
    /// if response.assigned {
    ///     println!("角色分配成功: assignment_id={}", response.assignment_id);
    /// }
    /// ```
    pub async fn assign_role(
        &self,
        request: AssignRoleRequest,
    ) -> SecurityResult<AssignRoleResponse> {
        let context = ErrorContext::new("assign_role")
            .with_user_id(&request.operator_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("role_id", &request.role_id);

        info!(
            "开始分配角色: operator={}, target={}, role={}",
            request.operator_id, request.target_user_id, request.role_id
        );

        // 验证请求参数
        validate_assign_role_request(&request)?;

        // 调用服务层
        match self.service.assign_role(request.clone()).await {
            Ok(response) => {
                info!(
                    "角色分配完成: assignment_id={}, target={}, role={}",
                    response.assignment_id, response.target_user_id, response.role_id
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "角色分配失败: target={}, role={}, error={}",
                    request.target_user_id, request.role_id, e
                );
                Err(e)
            }
        }
    }
}

// 构建器模式实现

/// 角色创建请求构建器
#[derive(Debug, Clone)]
pub struct CreateRoleRequestBuilder {
    request: CreateRoleRequest,
}

impl CreateRoleRequestBuilder {
    /// 创建新的角色创建请求构建器
    pub fn new() -> Self {
        Self {
            request: CreateRoleRequest {
                role_name: String::new(),
                description: None,
                permissions: Vec::new(),
                creator_id: String::new(),
                role_type: None,
            },
        }
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = role_name.into();
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request.permissions = permissions;
        self
    }

    /// 添加权限
    pub fn add_permission(mut self, permission: impl Into<String>) -> Self {
        self.request.permissions.push(permission.into());
        self
    }

    /// 设置创建者ID
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = creator_id.into();
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: RoleType) -> Self {
        self.request.role_type = Some(role_type);
        self
    }

    /// 构建角色创建请求
    pub fn build(self) -> SecurityResult<CreateRoleRequest> {
        validate_create_role_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for CreateRoleRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 角色删除请求构建器
#[derive(Debug, Clone)]
pub struct DeleteRoleRequestBuilder {
    request: DeleteRoleRequest,
}

impl DeleteRoleRequestBuilder {
    /// 创建新的角色删除请求构建器
    pub fn new() -> Self {
        Self {
            request: DeleteRoleRequest {
                role_id: String::new(),
                operator_id: String::new(),
            },
        }
    }

    /// 设置角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.request.role_id = role_id.into();
        self
    }

    /// 设置操作者ID
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.request.operator_id = operator_id.into();
        self
    }

    /// 构建角色删除请求
    pub fn build(self) -> SecurityResult<DeleteRoleRequest> {
        validate_delete_role_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for DeleteRoleRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 角色更新请求构建器
#[derive(Debug, Clone)]
pub struct UpdateRoleRequestBuilder {
    request: UpdateRoleRequest,
}

impl UpdateRoleRequestBuilder {
    /// 创建新的角色更新请求构建器
    pub fn new() -> Self {
        Self {
            request: UpdateRoleRequest {
                role_id: String::new(),
                role_name: None,
                description: None,
                permissions: None,
                operator_id: String::new(),
            },
        }
    }

    /// 设置角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.request.role_id = role_id.into();
        self
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = Some(role_name.into());
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request.permissions = Some(permissions);
        self
    }

    /// 添加权限
    pub fn add_permission(mut self, permission: impl Into<String>) -> Self {
        if self.request.permissions.is_none() {
            self.request.permissions = Some(Vec::new());
        }
        if let Some(ref mut permissions) = self.request.permissions {
            permissions.push(permission.into());
        }
        self
    }

    /// 设置操作者ID
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.request.operator_id = operator_id.into();
        self
    }

    /// 构建角色更新请求
    pub fn build(self) -> SecurityResult<UpdateRoleRequest> {
        validate_update_role_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for UpdateRoleRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 角色列表请求构建器
#[derive(Debug, Clone)]
pub struct ListRolesRequestBuilder {
    request: ListRolesRequest,
}

impl ListRolesRequestBuilder {
    /// 创建新的角色列表请求构建器
    pub fn new() -> Self {
        Self {
            request: ListRolesRequest {
                page_size: None,
                page_token: None,
                role_type: None,
                status: None,
            },
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 设置角色类型过滤
    pub fn role_type(mut self, role_type: RoleType) -> Self {
        self.request.role_type = Some(role_type);
        self
    }

    /// 设置状态过滤
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    /// 构建角色列表请求
    pub fn build(self) -> SecurityResult<ListRolesRequest> {
        validate_list_roles_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for ListRolesRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 角色分配请求构建器
#[derive(Debug, Clone)]
pub struct AssignRoleRequestBuilder {
    request: AssignRoleRequest,
}

impl AssignRoleRequestBuilder {
    /// 创建新的角色分配请求构建器
    pub fn new() -> Self {
        Self {
            request: AssignRoleRequest {
                operator_id: String::new(),
                target_user_id: String::new(),
                role_id: String::new(),
                expires_at: None,
                reason: None,
            },
        }
    }

    /// 设置操作者ID
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.request.operator_id = operator_id.into();
        self
    }

    /// 设置目标用户ID
    pub fn target_user_id(mut self, target_user_id: impl Into<String>) -> Self {
        self.request.target_user_id = target_user_id.into();
        self
    }

    /// 设置角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.request.role_id = role_id.into();
        self
    }

    /// 设置过期时间
    pub fn expires_at(mut self, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.request.expires_at = Some(expires_at);
        self
    }

    /// 设置分配原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.request.reason = Some(reason.into());
        self
    }

    /// 构建角色分配请求
    pub fn build(self) -> SecurityResult<AssignRoleRequest> {
        validate_assign_role_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for AssignRoleRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 验证函数

/// 验证角色创建请求
fn validate_create_role_request(request: &CreateRoleRequest) -> SecurityResult<()> {
    if request.role_name.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_name".to_string(),
            reason: "角色名称不能为空".to_string(),
        });
    }

    if request.role_name.len() > 100 {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_name".to_string(),
            reason: "角色名称长度不能超过100个字符".to_string(),
        });
    }

    // 验证角色名称格式
    if request
        .role_name
        .chars()
        .any(|c| !c.is_alphanumeric() && !matches!(c, '_' | '-' | ' '))
    {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_name".to_string(),
            reason: "角色名称只能包含字母、数字、下划线、连字符和空格".to_string(),
        });
    }

    if request.creator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "creator_id".to_string(),
            reason: "创建者ID不能为空".to_string(),
        });
    }

    if request.permissions.len() > 1000 {
        return Err(SecurityError::InvalidParameter {
            parameter: "permissions".to_string(),
            reason: "权限数量不能超过1000个".to_string(),
        });
    }

    // 验证每个权限的格式
    for (index, permission) in request.permissions.iter().enumerate() {
        if permission.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: format!("permissions[{}]", index),
                reason: "权限标识符不能为空".to_string(),
            });
        }

        if !permission.contains(':') {
            return Err(SecurityError::InvalidParameter {
                parameter: format!("permissions[{}]", index),
                reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
            });
        }
    }

    Ok(())
}

/// 验证角色删除请求
fn validate_delete_role_request(request: &DeleteRoleRequest) -> SecurityResult<()> {
    if request.role_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_id".to_string(),
            reason: "角色ID不能为空".to_string(),
        });
    }

    if request.operator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "operator_id".to_string(),
            reason: "操作者ID不能为空".to_string(),
        });
    }

    Ok(())
}

/// 验证角色更新请求
fn validate_update_role_request(request: &UpdateRoleRequest) -> SecurityResult<()> {
    if request.role_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_id".to_string(),
            reason: "角色ID不能为空".to_string(),
        });
    }

    if request.operator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "operator_id".to_string(),
            reason: "操作者ID不能为空".to_string(),
        });
    }

    // 验证角色名称
    if let Some(ref role_name) = request.role_name {
        if role_name.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_name".to_string(),
                reason: "角色名称不能为空".to_string(),
            });
        }

        if role_name.len() > 100 {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_name".to_string(),
                reason: "角色名称长度不能超过100个字符".to_string(),
            });
        }

        // 验证角色名称格式
        if role_name
            .chars()
            .any(|c| !c.is_alphanumeric() && !matches!(c, '_' | '-' | ' '))
        {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_name".to_string(),
                reason: "角色名称只能包含字母、数字、下划线、连字符和空格".to_string(),
            });
        }
    }

    // 验证权限列表
    if let Some(ref permissions) = request.permissions {
        if permissions.len() > 1000 {
            return Err(SecurityError::InvalidParameter {
                parameter: "permissions".to_string(),
                reason: "权限数量不能超过1000个".to_string(),
            });
        }

        // 验证每个权限的格式
        for (index, permission) in permissions.iter().enumerate() {
            if permission.is_empty() {
                return Err(SecurityError::InvalidParameter {
                    parameter: format!("permissions[{}]", index),
                    reason: "权限标识符不能为空".to_string(),
                });
            }

            if !permission.contains(':') {
                return Err(SecurityError::InvalidParameter {
                    parameter: format!("permissions[{}]", index),
                    reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
                });
            }
        }
    }

    Ok(())
}

/// 验证角色列表请求
fn validate_list_roles_request(request: &ListRolesRequest) -> SecurityResult<()> {
    if let Some(page_size) = request.page_size {
        if page_size == 0 || page_size > 1000 {
            return Err(SecurityError::InvalidParameter {
                parameter: "page_size".to_string(),
                reason: "分页大小必须在1-1000之间".to_string(),
            });
        }
    }

    if let Some(ref status) = request.status {
        let valid_statuses = vec!["active", "inactive", "suspended"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(SecurityError::InvalidParameter {
                parameter: "status".to_string(),
                reason: "状态值无效，应为 active、inactive 或 suspended".to_string(),
            });
        }
    }

    Ok(())
}

/// 验证角色分配请求
fn validate_assign_role_request(request: &AssignRoleRequest) -> SecurityResult<()> {
    if request.operator_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "operator_id".to_string(),
            reason: "操作者ID不能为空".to_string(),
        });
    }

    if request.target_user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "target_user_id".to_string(),
            reason: "目标用户ID不能为空".to_string(),
        });
    }

    if request.role_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "role_id".to_string(),
            reason: "角色ID不能为空".to_string(),
        });
    }

    // 验证过期时间不能是过去的时间
    if let Some(expires_at) = request.expires_at {
        if expires_at < chrono::Utc::now() {
            return Err(SecurityError::InvalidParameter {
                parameter: "expires_at".to_string(),
                reason: "过期时间不能是过去的时间".to_string(),
            });
        }
    }

    Ok(())
}

// 便捷函数

/// 创建角色创建请求
pub fn create_create_role_request(
    role_name: impl Into<String>,
    creator_id: impl Into<String>,
) -> CreateRoleRequestBuilder {
    CreateRoleRequestBuilder::new()
        .role_name(role_name)
        .creator_id(creator_id)
}

/// 创建角色删除请求
pub fn create_delete_role_request(
    role_id: impl Into<String>,
    operator_id: impl Into<String>,
) -> DeleteRoleRequestBuilder {
    DeleteRoleRequestBuilder::new()
        .role_id(role_id)
        .operator_id(operator_id)
}

/// 创建角色更新请求
pub fn create_update_role_request(
    role_id: impl Into<String>,
    operator_id: impl Into<String>,
) -> UpdateRoleRequestBuilder {
    UpdateRoleRequestBuilder::new()
        .role_id(role_id)
        .operator_id(operator_id)
}

/// 创建角色列表请求
pub fn create_list_roles_request() -> ListRolesRequestBuilder {
    ListRolesRequestBuilder::new()
}

/// 创建角色分配请求
pub fn create_assign_role_request(
    operator_id: impl Into<String>,
    target_user_id: impl Into<String>,
    role_id: impl Into<String>,
) -> AssignRoleRequestBuilder {
    AssignRoleRequestBuilder::new()
        .operator_id(operator_id)
        .target_user_id(target_user_id)
        .role_id(role_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acs::service::DefaultAccessControlService;

    #[tokio::test]
    async fn test_create_role() {
        let api = RoleV1API::default();
        let request = CreateRoleRequestBuilder::new()
            .role_name("测试角色")
            .description("这是一个测试角色")
            .add_permission("document:read")
            .add_permission("document:write")
            .creator_id("admin_001")
            .role_type(RoleType::Business)
            .build()
            .unwrap();

        let response = api.create_role(request).await.unwrap();
        assert!(!response.role_id.is_empty());
        assert_eq!(response.role_name, "测试角色");
        assert_eq!(response.creator_id, "admin_001");
    }

    #[tokio::test]
    async fn test_list_roles() {
        let api = RoleV1API::default();
        let request = ListRolesRequestBuilder::new()
            .page_size(10)
            .role_type(RoleType::Business)
            .status("active")
            .build()
            .unwrap();

        let response = api.list_roles(request).await.unwrap();
        assert!(response.total_count > 0);
        assert!(!response.roles.is_empty());
    }

    #[tokio::test]
    async fn test_assign_role() {
        let api = RoleV1API::default();
        let request = AssignRoleRequestBuilder::new()
            .operator_id("admin_001")
            .target_user_id("user_001")
            .role_id("role_001")
            .reason("工作需要")
            .build()
            .unwrap();

        let response = api.assign_role(request).await.unwrap();
        assert!(response.assigned);
        assert_eq!(response.target_user_id, "user_001");
        assert_eq!(response.role_id, "role_001");
    }

    #[test]
    fn test_role_request_builders() {
        // 测试角色创建请求构建器
        let request = CreateRoleRequestBuilder::new()
            .role_name("管理员")
            .add_permission("user:*")
            .creator_id("admin_001")
            .build();
        assert!(request.is_ok());

        // 测试角色分配请求构建器
        let request = AssignRoleRequestBuilder::new()
            .operator_id("admin_001")
            .target_user_id("user_001")
            .role_id("role_001")
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_validation() {
        // 测试空角色名称
        let request = CreateRoleRequestBuilder::new()
            .creator_id("admin_001")
            .build();
        assert!(request.is_err());

        // 测试无效角色名称（包含特殊字符）
        let request = CreateRoleRequestBuilder::new()
            .role_name("test@role")
            .creator_id("admin_001")
            .build();
        assert!(request.is_err());

        // 测试过长角色名称
        let long_name = "a".repeat(101);
        let request = CreateRoleRequestBuilder::new()
            .role_name(long_name)
            .creator_id("admin_001")
            .build();
        assert!(request.is_err());
    }
}
