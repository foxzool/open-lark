//! 权限管理 V1 API
//!
//! 提供用户权限检查、授权、撤销和权限列表管理功能。

use crate::acs::service::{
    AccessControlService, BatchPermissionCheckRequest, BatchPermissionCheckResponse,
    GrantPermissionRequest, GrantPermissionResponse, PermissionCheckRequest,
    PermissionCheckResponse, RevokePermissionRequest, RevokePermissionResponse, UserPermission,
    UserPermissionListRequest, UserPermissionListResponse,
};
use crate::error::{ErrorContext, SecurityError, SecurityResult};
use crate::models::{DeviceInfo, DeviceType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

/// 权限管理API v1
pub struct PermissionV1API {
    service: Box<dyn AccessControlService>,
}

impl PermissionV1API {
    /// 创建新的权限API实例
    pub fn new(service: Box<dyn AccessControlService>) -> Self {
        Self { service }
    }

    /// 创建默认权限API实例
    pub fn default() -> Self {
        Self::new(Box::new(
            crate::acs::service::DefaultAccessControlService::with_default_config(),
        ))
    }
}

impl PermissionV1API {
    /// 检查用户权限
    ///
    /// # 参数
    /// - `request`: 权限检查请求
    ///
    /// # 返回值
    /// 返回权限检查结果，包含是否具有权限的详细信息
    ///
    /// # 示例
    /// ```rust
    /// let api = PermissionV1API::default();
    /// let request = PermissionCheckRequestBuilder::new()
    ///     .user_id("user_001")
    ///     .permission("document:read")
    ///     .resource("doc_123")
    ///     .build()?;
    /// let response = api.check_permission(request).await?;
    /// if response.has_permission {
    ///     println!("用户具有权限");
    /// }
    /// ```
    pub async fn check_permission(
        &self,
        request: PermissionCheckRequest,
    ) -> SecurityResult<PermissionCheckResponse> {
        let context = ErrorContext::new("check_permission")
            .with_user_id(&request.user_id)
            .with_extra("permission", &request.permission)
            .with_extra("resource", &request.resource);

        info!(
            "开始检查用户权限: user_id={}, permission={}",
            request.user_id, request.permission
        );

        // 验证请求参数
        validate_permission_check_request(&request)?;

        // 调用服务层
        match self.service.check_permission(request.clone()).await {
            Ok(response) => {
                info!(
                    "权限检查完成: user_id={}, has_permission={}",
                    response.user_id, response.has_permission
                );
                Ok(response)
            }
            Err(e) => {
                error!("权限检查失败: user_id={}, error={}", request.user_id, e);
                Err(e)
            }
        }
    }

    /// 授予权限
    ///
    /// # 参数
    /// - `request`: 权限授予请求
    ///
    /// # 返回值
    /// 返回权限授予结果，包含权限ID和授予时间
    ///
    /// # 示例
    /// ```rust
    /// let api = PermissionV1API::default();
    /// let request = GrantPermissionRequestBuilder::new()
    ///     .granter_id("admin_001")
    ///     .target_user_id("user_001")
    ///     .permission("document:read")
    ///     .resource_type("document")
    ///     .build()?;
    /// let response = api.grant_permission(request).await?;
    /// if response.granted {
    ///     println!("权限授予成功: permission_id={}", response.permission_id);
    /// }
    /// ```
    pub async fn grant_permission(
        &self,
        request: GrantPermissionRequest,
    ) -> SecurityResult<GrantPermissionResponse> {
        let context = ErrorContext::new("grant_permission")
            .with_user_id(&request.granter_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("permission", &request.permission);

        info!(
            "开始授予用户权限: granter={}, target={}, permission={}",
            request.granter_id, request.target_user_id, request.permission
        );

        // 验证请求参数
        validate_grant_permission_request(&request)?;

        // 调用服务层
        match self.service.grant_permission(request.clone()).await {
            Ok(response) => {
                info!(
                    "权限授予完成: permission_id={}, target={}",
                    response.permission_id, response.target_user_id
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "权限授予失败: target={}, error={}",
                    request.target_user_id, e
                );
                Err(e)
            }
        }
    }

    /// 撤销权限
    ///
    /// # 参数
    /// - `request`: 权限撤销请求
    ///
    /// # 返回值
    /// 返回权限撤销结果
    ///
    /// # 示例
    /// ```rust
    /// let api = PermissionV1API::default();
    /// let request = RevokePermissionRequestBuilder::new()
    ///     .revoker_id("admin_001")
    ///     .target_user_id("user_001")
    ///     .permission("document:read")
    /// .build()?;
    /// let response = api.revoke_permission(request).await?;
    /// if response.revoked {
    ///     println!("权限撤销成功");
    /// }
    /// ```
    pub async fn revoke_permission(
        &self,
        request: RevokePermissionRequest,
    ) -> SecurityResult<RevokePermissionResponse> {
        let context = ErrorContext::new("revoke_permission")
            .with_user_id(&request.revoker_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("permission", &request.permission);

        info!(
            "开始撤销用户权限: revoker={}, target={}, permission={}",
            request.revoker_id, request.target_user_id, request.permission
        );

        // 验证请求参数
        validate_revoke_permission_request(&request)?;

        // 调用服务层
        match self.service.revoke_permission(request.clone()).await {
            Ok(response) => {
                info!(
                    "权限撤销完成: target={}, permission={}",
                    response.target_user_id, response.permission
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "权限撤销失败: target={}, error={}",
                    request.target_user_id, e
                );
                Err(e)
            }
        }
    }

    /// 获取用户权限列表
    ///
    /// # 参数
    /// - `request`: 用户权限列表请求
    ///
    /// # 返回值
    /// 返回用户的所有权限列表
    ///
    /// # 示例
    /// ```rust
    /// let api = PermissionV1API::default();
    /// let request = UserPermissionListRequestBuilder::new()
    ///     .user_id("user_001")
    ///     .page_size(20)
    ///     .build()?;
    /// let response = api.list_user_permissions(request).await?;
    /// println!("用户权限数量: {}", response.total_count);
    /// for permission in response.permissions {
    ///     println!("权限: {}", permission.permission);
    /// }
    /// ```
    pub async fn list_user_permissions(
        &self,
        request: UserPermissionListRequest,
    ) -> SecurityResult<UserPermissionListResponse> {
        let context = ErrorContext::new("list_user_permissions").with_user_id(&request.user_id);

        info!("开始获取用户权限列表: user_id={}", request.user_id);

        // 验证请求参数
        validate_user_permission_list_request(&request)?;

        // 调用服务层
        match self.service.list_user_permissions(request.clone()).await {
            Ok(response) => {
                info!(
                    "用户权限列表获取完成: user_id={}, count={}",
                    response.user_id, response.total_count
                );
                Ok(response)
            }
            Err(e) => {
                error!(
                    "用户权限列表获取失败: user_id={}, error={}",
                    request.user_id, e
                );
                Err(e)
            }
        }
    }

    /// 批量检查权限
    ///
    /// # 参数
    /// - `request`: 批量权限检查请求
    ///
    /// # 返回值
    /// 返回批量权限检查结果
    ///
    /// # 示例
    /// ```rust
    /// let api = PermissionV1API::default();
    /// let permissions = vec!["document:read".to_string(), "document:write".to_string()];
    /// let request = BatchPermissionCheckRequestBuilder::new()
    ///     .user_id("user_001")
    ///     .permissions(permissions)
    ///     .resource("doc_123")
    ///     .build()?;
    /// let response = api.batch_check_permissions(request).await?;
    /// for result in response.results {
    ///     println!("权限 {}: {}", result.permission, if result.allowed { "允许" } else { "拒绝" });
    /// }
    /// ```
    pub async fn batch_check_permissions(
        &self,
        request: BatchPermissionCheckRequest,
    ) -> SecurityResult<BatchPermissionCheckResponse> {
        let context = ErrorContext::new("batch_check_permissions")
            .with_user_id(&request.user_id)
            .with_extra("permission_count", &request.permissions.len().to_string());

        info!(
            "开始批量检查权限: user_id={}, count={}",
            request.user_id,
            request.permissions.len()
        );

        // 验证请求参数
        validate_batch_permission_check_request(&request)?;

        // 调用服务层
        match self.service.batch_check_permissions(request.clone()).await {
            Ok(response) => {
                info!(
                    "批量权限检查完成: user_id={}, result_count={}",
                    response.user_id,
                    response.results.len()
                );
                Ok(response)
            }
            Err(e) => {
                error!("批量权限检查失败: user_id={}, error={}", request.user_id, e);
                Err(e)
            }
        }
    }
}

// 构建器模式实现

/// 权限检查请求构建器
#[derive(Debug, Clone)]
pub struct PermissionCheckRequestBuilder {
    request: PermissionCheckRequest,
}

impl PermissionCheckRequestBuilder {
    /// 创建新的权限检查请求构建器
    pub fn new() -> Self {
        Self {
            request: PermissionCheckRequest {
                user_id: String::new(),
                permission: String::new(),
                resource: String::new(),
                context: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
        self
    }

    /// 设置权限标识符
    pub fn permission(mut self, permission: impl Into<String>) -> Self {
        self.request.permission = permission.into();
        self
    }

    /// 设置资源标识符
    pub fn resource(mut self, resource: impl Into<String>) -> Self {
        self.request.resource = resource.into();
        self
    }

    /// 设置上下文信息
    pub fn context(mut self, context: HashMap<String, String>) -> Self {
        self.request.context = Some(context);
        self
    }

    /// 添加上下文信息
    pub fn add_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.request.context.is_none() {
            self.request.context = Some(HashMap::new());
        }
        if let Some(ref mut context) = self.request.context {
            context.insert(key.into(), value.into());
        }
        self
    }

    /// 构建权限检查请求
    pub fn build(self) -> SecurityResult<PermissionCheckRequest> {
        validate_permission_check_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for PermissionCheckRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 权限授予请求构建器
#[derive(Debug, Clone)]
pub struct GrantPermissionRequestBuilder {
    request: GrantPermissionRequest,
}

impl GrantPermissionRequestBuilder {
    /// 创建新的权限授予请求构建器
    pub fn new() -> Self {
        Self {
            request: GrantPermissionRequest {
                granter_id: String::new(),
                target_user_id: String::new(),
                permission: String::new(),
                resource_type: None,
                expires_at: None,
                reason: None,
            },
        }
    }

    /// 设置授权者ID
    pub fn granter_id(mut self, granter_id: impl Into<String>) -> Self {
        self.request.granter_id = granter_id.into();
        self
    }

    /// 设置目标用户ID
    pub fn target_user_id(mut self, target_user_id: impl Into<String>) -> Self {
        self.request.target_user_id = target_user_id.into();
        self
    }

    /// 设置权限标识符
    pub fn permission(mut self, permission: impl Into<String>) -> Self {
        self.request.permission = permission.into();
        self
    }

    /// 设置资源类型
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.request.resource_type = Some(resource_type.into());
        self
    }

    /// 设置过期时间
    pub fn expires_at(mut self, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.request.expires_at = Some(expires_at);
        self
    }

    /// 设置授权原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.request.reason = Some(reason.into());
        self
    }

    /// 构建权限授予请求
    pub fn build(self) -> SecurityResult<GrantPermissionRequest> {
        validate_grant_permission_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for GrantPermissionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 权限撤销请求构建器
#[derive(Debug, Clone)]
pub struct RevokePermissionRequestBuilder {
    request: RevokePermissionRequest,
}

impl RevokePermissionRequestBuilder {
    /// 创建新的权限撤销请求构建器
    pub fn new() -> Self {
        Self {
            request: RevokePermissionRequest {
                revoker_id: String::new(),
                target_user_id: String::new(),
                permission: String::new(),
                reason: None,
            },
        }
    }

    /// 设置撤销者ID
    pub fn revoker_id(mut self, revoker_id: impl Into<String>) -> Self {
        self.request.revoker_id = revoker_id.into();
        self
    }

    /// 设置目标用户ID
    pub fn target_user_id(mut self, target_user_id: impl Into<String>) -> Self {
        self.request.target_user_id = target_user_id.into();
        self
    }

    /// 设置权限标识符
    pub fn permission(mut self, permission: impl Into<String>) -> Self {
        self.request.permission = permission.into();
        self
    }

    /// 设置撤销原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.request.reason = Some(reason.into());
        self
    }

    /// 构建权限撤销请求
    pub fn build(self) -> SecurityResult<RevokePermissionRequest> {
        validate_revoke_permission_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for RevokePermissionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户权限列表请求构建器
#[derive(Debug, Clone)]
pub struct UserPermissionListRequestBuilder {
    request: UserPermissionListRequest,
}

impl UserPermissionListRequestBuilder {
    /// 创建新的用户权限列表请求构建器
    pub fn new() -> Self {
        Self {
            request: UserPermissionListRequest {
                user_id: String::new(),
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
        self
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

    /// 构建用户权限列表请求
    pub fn build(self) -> SecurityResult<UserPermissionListRequest> {
        validate_user_permission_list_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for UserPermissionListRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 批量权限检查请求构建器
#[derive(Debug, Clone)]
pub struct BatchPermissionCheckRequestBuilder {
    request: BatchPermissionCheckRequest,
}

impl BatchPermissionCheckRequestBuilder {
    /// 创建新的批量权限检查请求构建器
    pub fn new() -> Self {
        Self {
            request: BatchPermissionCheckRequest {
                user_id: String::new(),
                permissions: Vec::new(),
                resource: None,
                context: None,
            },
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
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

    /// 设置资源标识符
    pub fn resource(mut self, resource: impl Into<String>) -> Self {
        self.request.resource = Some(resource.into());
        self
    }

    /// 设置上下文信息
    pub fn context(mut self, context: HashMap<String, String>) -> Self {
        self.request.context = Some(context);
        self
    }

    /// 添加上下文信息
    pub fn add_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.request.context.is_none() {
            self.request.context = Some(HashMap::new());
        }
        if let Some(ref mut context) = self.request.context {
            context.insert(key.into(), value.into());
        }
        self
    }

    /// 构建批量权限检查请求
    pub fn build(self) -> SecurityResult<BatchPermissionCheckRequest> {
        validate_batch_permission_check_request(&self.request)?;
        Ok(self.request)
    }
}

impl Default for BatchPermissionCheckRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 验证函数

/// 验证权限检查请求
fn validate_permission_check_request(request: &PermissionCheckRequest) -> SecurityResult<()> {
    if request.user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "user_id".to_string(),
            reason: "用户ID不能为空".to_string(),
        });
    }

    if request.permission.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "permission".to_string(),
            reason: "权限标识符不能为空".to_string(),
        });
    }

    if request.resource.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "resource".to_string(),
            reason: "资源标识符不能为空".to_string(),
        });
    }

    // 验证权限格式
    if !request.permission.contains(':') {
        return Err(SecurityError::InvalidParameter {
            parameter: "permission".to_string(),
            reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
        });
    }

    Ok(())
}

/// 验证权限授予请求
fn validate_grant_permission_request(request: &GrantPermissionRequest) -> SecurityResult<()> {
    if request.granter_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "granter_id".to_string(),
            reason: "授权者ID不能为空".to_string(),
        });
    }

    if request.target_user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "target_user_id".to_string(),
            reason: "目标用户ID不能为空".to_string(),
        });
    }

    if request.permission.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "permission".to_string(),
            reason: "权限标识符不能为空".to_string(),
        });
    }

    // 验证权限格式
    if !request.permission.contains(':') {
        return Err(SecurityError::InvalidParameter {
            parameter: "permission".to_string(),
            reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
        });
    }

    Ok(())
}

/// 验证权限撤销请求
fn validate_revoke_permission_request(request: &RevokePermissionRequest) -> SecurityResult<()> {
    if request.revoker_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "revoker_id".to_string(),
            reason: "撤销者ID不能为空".to_string(),
        });
    }

    if request.target_user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "target_user_id".to_string(),
            reason: "目标用户ID不能为空".to_string(),
        });
    }

    if request.permission.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "permission".to_string(),
            reason: "权限标识符不能为空".to_string(),
        });
    }

    Ok(())
}

/// 验证用户权限列表请求
fn validate_user_permission_list_request(
    request: &UserPermissionListRequest,
) -> SecurityResult<()> {
    if request.user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "user_id".to_string(),
            reason: "用户ID不能为空".to_string(),
        });
    }

    if let Some(page_size) = request.page_size {
        if page_size == 0 || page_size > 1000 {
            return Err(SecurityError::InvalidParameter {
                parameter: "page_size".to_string(),
                reason: "分页大小必须在1-1000之间".to_string(),
            });
        }
    }

    Ok(())
}

/// 验证批量权限检查请求
fn validate_batch_permission_check_request(
    request: &BatchPermissionCheckRequest,
) -> SecurityResult<()> {
    if request.user_id.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "user_id".to_string(),
            reason: "用户ID不能为空".to_string(),
        });
    }

    if request.permissions.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "permissions".to_string(),
            reason: "权限列表不能为空".to_string(),
        });
    }

    if request.permissions.len() > 100 {
        return Err(SecurityError::InvalidParameter {
            parameter: "permissions".to_string(),
            reason: "权限列表数量不能超过100个".to_string(),
        });
    }

    // 验证每个权限的格式
    for permission in &request.permissions {
        if permission.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "permission".to_string(),
                reason: "权限标识符不能为空".to_string(),
            });
        }

        if !permission.contains(':') {
            return Err(SecurityError::InvalidParameter {
                parameter: "permission".to_string(),
                reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
            });
        }
    }

    Ok(())
}

// 便捷函数

/// 创建权限检查请求
pub fn create_permission_check_request(
    user_id: impl Into<String>,
    permission: impl Into<String>,
    resource: impl Into<String>,
) -> PermissionCheckRequestBuilder {
    PermissionCheckRequestBuilder::new()
        .user_id(user_id)
        .permission(permission)
        .resource(resource)
}

/// 创建权限授予请求
pub fn create_grant_permission_request(
    granter_id: impl Into<String>,
    target_user_id: impl Into<String>,
    permission: impl Into<String>,
) -> GrantPermissionRequestBuilder {
    GrantPermissionRequestBuilder::new()
        .granter_id(granter_id)
        .target_user_id(target_user_id)
        .permission(permission)
}

/// 创建权限撤销请求
pub fn create_revoke_permission_request(
    revoker_id: impl Into<String>,
    target_user_id: impl Into<String>,
    permission: impl Into<String>,
) -> RevokePermissionRequestBuilder {
    RevokePermissionRequestBuilder::new()
        .revoker_id(revoker_id)
        .target_user_id(target_user_id)
        .permission(permission)
}

/// 创建用户权限列表请求
pub fn create_user_permission_list_request(
    user_id: impl Into<String>,
) -> UserPermissionListRequestBuilder {
    UserPermissionListRequestBuilder::new().user_id(user_id)
}

/// 创建批量权限检查请求
pub fn create_batch_permission_check_request(
    user_id: impl Into<String>,
    permissions: Vec<String>,
) -> BatchPermissionCheckRequestBuilder {
    BatchPermissionCheckRequestBuilder::new()
        .user_id(user_id)
        .permissions(permissions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acs::service::DefaultAccessControlService;

    #[tokio::test]
    async fn test_check_permission() {
        let api = PermissionV1API::default();
        let request = PermissionCheckRequestBuilder::new()
            .user_id("test_user")
            .permission("document:read")
            .resource("doc_123")
            .build()
            .unwrap();

        let response = api.check_permission(request).await.unwrap();
        assert_eq!(response.user_id, "test_user");
        assert_eq!(response.permission, "document:read");
    }

    #[tokio::test]
    async fn test_grant_permission() {
        let api = PermissionV1API::default();
        let request = GrantPermissionRequestBuilder::new()
            .granter_id("admin_001")
            .target_user_id("user_001")
            .permission("document:read")
            .build()
            .unwrap();

        let response = api.grant_permission(request).await.unwrap();
        assert!(response.granted);
        assert_eq!(response.target_user_id, "user_001");
    }

    #[test]
    fn test_permission_check_request_builder() {
        let request = PermissionCheckRequestBuilder::new()
            .user_id("test_user")
            .permission("document:read")
            .resource("doc_123")
            .add_context("ip_address", "192.168.1.1")
            .build()
            .unwrap();

        assert_eq!(request.user_id, "test_user");
        assert_eq!(request.permission, "document:read");
        assert_eq!(request.resource, "doc_123");
        assert!(request.context.is_some());
    }

    #[test]
    fn test_validation() {
        // 测试空用户ID
        let request = PermissionCheckRequestBuilder::new()
            .permission("document:read")
            .resource("doc_123")
            .build();
        assert!(request.is_err());

        // 测试错误权限格式
        let request = PermissionCheckRequestBuilder::new()
            .user_id("test_user")
            .permission("invalid_permission")
            .resource("doc_123")
            .build();
        assert!(request.is_err());
    }
}
