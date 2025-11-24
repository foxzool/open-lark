//! 访问控制服务模块
//!
//! 提供基于角色的访问控制（RBAC）和基于属性的访问控制（ABAC）功能。

use crate::error::{ErrorContext, SecurityError, SecurityResult};
use crate::models::DeviceInfo;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

/// 访问控制服务特征
#[async_trait]
pub trait AccessControlService: Send + Sync {
    /// 权限检查请求
    async fn check_permission(
        &self,
        request: PermissionCheckRequest,
    ) -> SecurityResult<PermissionCheckResponse>;

    /// 创建策略请求
    async fn create_policy(
        &self,
        request: CreatePolicyRequest,
    ) -> SecurityResult<CreatePolicyResponse>;

    /// 删除策略请求
    async fn delete_policy(
        &self,
        request: DeletePolicyRequest,
    ) -> SecurityResult<DeletePolicyResponse>;

    /// 更新策略请求
    async fn update_policy(
        &self,
        request: UpdatePolicyRequest,
    ) -> SecurityResult<UpdatePolicyResponse>;

    /// 策略评估请求
    async fn evaluate_policy(
        &self,
        request: PolicyEvaluationRequest,
    ) -> SecurityResult<PolicyEvaluationResponse>;

    /// 授权权限请求
    async fn grant_permission(
        &self,
        request: GrantPermissionRequest,
    ) -> SecurityResult<GrantPermissionResponse>;

    /// 撤销权限请求
    async fn revoke_permission(
        &self,
        request: RevokePermissionRequest,
    ) -> SecurityResult<RevokePermissionResponse>;

    /// 获取用户权限列表
    async fn list_user_permissions(
        &self,
        request: UserPermissionListRequest,
    ) -> SecurityResult<UserPermissionListResponse>;

    /// 批量权限检查
    async fn batch_check_permissions(
        &self,
        request: BatchPermissionCheckRequest,
    ) -> SecurityResult<BatchPermissionCheckResponse>;

    /// 创建角色
    async fn create_role(&self, request: CreateRoleRequest) -> SecurityResult<CreateRoleResponse>;

    /// 删除角色
    async fn delete_role(&self, request: DeleteRoleRequest) -> SecurityResult<DeleteRoleResponse>;

    /// 更新角色
    async fn update_role(&self, request: UpdateRoleRequest) -> SecurityResult<UpdateRoleResponse>;

    /// 获取角色列表
    async fn list_roles(&self, request: ListRolesRequest) -> SecurityResult<ListRolesResponse>;

    /// 分配角色给用户
    async fn assign_role(&self, request: AssignRoleRequest) -> SecurityResult<AssignRoleResponse>;
}

/// 默认访问控制服务实现
#[derive(Clone, Debug)]
pub struct DefaultAccessControlService {
    /// 服务配置
    config: AccessControlConfig,
}

/// 访问控制配置
#[derive(Debug, Clone)]
pub struct AccessControlConfig {
    /// 启用缓存
    pub enable_cache: bool,
    /// 缓存过期时间（秒）
    pub cache_ttl: u64,
    /// 启用审计日志
    pub enable_audit_log: bool,
    /// 默认策略
    pub default_policy: DefaultPolicy,
    /// 权限继承深度限制
    pub max_inheritance_depth: u32,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enable_cache: true,
            cache_ttl: 300, // 5分钟
            enable_audit_log: true,
            default_policy: DefaultPolicy::Deny,
            max_inheritance_depth: 10,
        }
    }
}

/// 默认策略类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefaultPolicy {
    /// 默认允许
    Allow,
    /// 默认拒绝
    Deny,
    /// 需要显式授权
    RequireExplicit,
}

impl DefaultAccessControlService {
    /// 创建新的访问控制服务
    pub fn new(config: AccessControlConfig) -> Self {
        Self { config }
    }

    /// 创建默认配置的访问控制服务
    pub fn with_default_config() -> Self {
        Self::new(AccessControlConfig::default())
    }

    /// 验证权限格式
    fn validate_permission(&self, permission: &str) -> SecurityResult<()> {
        if permission.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "permission".to_string(),
                reason: "权限标识符不能为空".to_string(),
            });
        }

        if permission.len() > 255 {
            return Err(SecurityError::InvalidParameter {
                parameter: "permission".to_string(),
                reason: "权限标识符长度不能超过255个字符".to_string(),
            });
        }

        // 验证权限格式（如：resource:action）
        if !permission.contains(':') {
            return Err(SecurityError::InvalidParameter {
                parameter: "permission".to_string(),
                reason: "权限标识符格式错误，应为 'resource:action'".to_string(),
            });
        }

        Ok(())
    }

    /// 验证角色名称
    fn validate_role_name(&self, role_name: &str) -> SecurityResult<()> {
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

        Ok(())
    }

    /// 记录访问控制审计日志
    fn log_access_control_event(
        &self,
        user_id: &str,
        resource: &str,
        action: &str,
        result: bool,
        reason: Option<&str>,
    ) {
        if self.config.enable_audit_log {
            let log_level = if result {
                tracing::Level::INFO
            } else {
                tracing::Level::WARN
            };

            let message = format!(
                "访问控制事件 - 用户: {}, 资源: {}, 操作: {}, 结果: {}",
                user_id,
                resource,
                action,
                if result { "允许" } else { "拒绝" }
            );

            if let Some(reason) = reason {
                info!(target: "access_control", "{} - 原因: {}", message, reason);
            } else {
                info!(target: "access_control", "{}", message);
            }
        }
    }
}

#[async_trait]
impl AccessControlService for DefaultAccessControlService {
    async fn check_permission(
        &self,
        request: PermissionCheckRequest,
    ) -> SecurityResult<PermissionCheckResponse> {
        let context = ErrorContext::new("check_permission")
            .with_user_id(&request.user_id)
            .with_extra("permission", &request.permission)
            .with_extra("resource", &request.resource);

        debug!(
            "检查用户权限: user_id={}, permission={}",
            request.user_id, request.permission
        );

        // 验证请求参数
        self.validate_permission(&request.permission)?;

        // 模拟权限检查逻辑
        // 在实际实现中，这里会查询数据库或权限系统
        let has_permission =
            request.permission == "admin:read" || request.permission == "user:read";

        // 记录审计日志
        self.log_access_control_event(
            &request.user_id,
            &request.resource,
            "check_permission",
            has_permission,
            if has_permission {
                None
            } else {
                Some("权限不足")
            },
        );

        let response = PermissionCheckResponse {
            has_permission,
            user_id: request.user_id,
            permission: request.permission,
            resource: request.resource,
            reason: if has_permission {
                Some("权限验证通过".to_string())
            } else {
                Some("权限不足".to_string())
            },
            evaluated_at: chrono::Utc::now(),
        };

        info!(
            "权限检查完成: user_id={}, has_permission={}",
            response.user_id, response.has_permission
        );
        Ok(response)
    }

    async fn create_policy(
        &self,
        request: CreatePolicyRequest,
    ) -> SecurityResult<CreatePolicyResponse> {
        let context = ErrorContext::new("create_policy")
            .with_user_id(&request.creator_id)
            .with_extra("policy_name", &request.policy_name);

        debug!(
            "创建访问控制策略: name={}, creator={}",
            request.policy_name, request.creator_id
        );

        // 验证策略名称
        if request.policy_name.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_name".to_string(),
                reason: "策略名称不能为空".to_string(),
            });
        }

        // 模拟策略创建
        let policy_id = uuid::Uuid::new_v4().to_string();

        let response = CreatePolicyResponse {
            policy_id: policy_id.clone(),
            policy_name: request.policy_name,
            creator_id: request.creator_id,
            created_at: chrono::Utc::now(),
            status: "active".to_string(),
        };

        info!(
            "访问控制策略创建成功: policy_id={}, name={}",
            policy_id, response.policy_name
        );
        Ok(response)
    }

    async fn delete_policy(
        &self,
        request: DeletePolicyRequest,
    ) -> SecurityResult<DeletePolicyResponse> {
        let context = ErrorContext::new("delete_policy")
            .with_user_id(&request.operator_id)
            .with_extra("policy_id", &request.policy_id);

        debug!(
            "删除访问控制策略: policy_id={}, operator={}",
            request.policy_id, request.operator_id
        );

        // 验证策略ID
        if request.policy_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_id".to_string(),
                reason: "策略ID不能为空".to_string(),
            });
        }

        let response = DeletePolicyResponse {
            policy_id: request.policy_id.clone(),
            deleted: true,
            operator_id: request.operator_id,
            deleted_at: chrono::Utc::now(),
        };

        info!("访问控制策略删除成功: policy_id={}", response.policy_id);
        Ok(response)
    }

    async fn update_policy(
        &self,
        request: UpdatePolicyRequest,
    ) -> SecurityResult<UpdatePolicyResponse> {
        let context = ErrorContext::new("update_policy")
            .with_user_id(&request.operator_id)
            .with_extra("policy_id", &request.policy_id);

        debug!(
            "更新访问控制策略: policy_id={}, operator={}",
            request.policy_id, request.operator_id
        );

        // 验证策略ID
        if request.policy_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "policy_id".to_string(),
                reason: "策略ID不能为空".to_string(),
            });
        }

        let response = UpdatePolicyResponse {
            policy_id: request.policy_id.clone(),
            updated: true,
            operator_id: request.operator_id,
            updated_at: chrono::Utc::now(),
        };

        info!("访问控制策略更新成功: policy_id={}", response.policy_id);
        Ok(response)
    }

    async fn evaluate_policy(
        &self,
        request: PolicyEvaluationRequest,
    ) -> SecurityResult<PolicyEvaluationResponse> {
        let context = ErrorContext::new("evaluate_policy")
            .with_user_id(&request.user_id)
            .with_extra("resource", &request.resource)
            .with_extra("action", &request.action);

        debug!(
            "评估策略: user_id={}, resource={}, action={}",
            request.user_id, request.resource, request.action
        );

        // 模拟策略评估
        let allowed = request.action == "read"
            || (request.action == "write" && request.user_id.starts_with("admin"));

        let response = PolicyEvaluationResponse {
            allowed,
            user_id: request.user_id,
            resource: request.resource,
            action: request.action,
            evaluated_policies: vec!["policy_001".to_string()],
            reason: if allowed {
                Some("策略评估通过".to_string())
            } else {
                Some("策略评估拒绝".to_string())
            },
            evaluated_at: chrono::Utc::now(),
        };

        info!(
            "策略评估完成: user_id={}, allowed={}",
            response.user_id, response.allowed
        );
        Ok(response)
    }

    async fn grant_permission(
        &self,
        request: GrantPermissionRequest,
    ) -> SecurityResult<GrantPermissionResponse> {
        let context = ErrorContext::new("grant_permission")
            .with_user_id(&request.granter_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("permission", &request.permission);

        debug!(
            "授权权限: granter={}, target={}, permission={}",
            request.granter_id, request.target_user_id, request.permission
        );

        // 验证权限格式
        self.validate_permission(&request.permission)?;

        let response = GrantPermissionResponse {
            granted: true,
            permission_id: uuid::Uuid::new_v4().to_string(),
            granter_id: request.granter_id,
            target_user_id: request.target_user_id,
            permission: request.permission,
            granted_at: chrono::Utc::now(),
        };

        info!(
            "权限授权成功: permission_id={}, target={}",
            response.permission_id, response.target_user_id
        );
        Ok(response)
    }

    async fn revoke_permission(
        &self,
        request: RevokePermissionRequest,
    ) -> SecurityResult<RevokePermissionResponse> {
        let context = ErrorContext::new("revoke_permission")
            .with_user_id(&request.revoker_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("permission", &request.permission);

        debug!(
            "撤销权限: revoker={}, target={}, permission={}",
            request.revoker_id, request.target_user_id, request.permission
        );

        let response = RevokePermissionResponse {
            revoked: true,
            revoker_id: request.revoker_id,
            target_user_id: request.target_user_id,
            permission: request.permission,
            revoked_at: chrono::Utc::now(),
        };

        info!(
            "权限撤销成功: target={}, permission={}",
            response.target_user_id, response.permission
        );
        Ok(response)
    }

    async fn list_user_permissions(
        &self,
        request: UserPermissionListRequest,
    ) -> SecurityResult<UserPermissionListResponse> {
        let context = ErrorContext::new("list_user_permissions").with_user_id(&request.user_id);

        debug!("获取用户权限列表: user_id={}", request.user_id);

        // 模拟权限列表
        let permissions = vec![
            UserPermission {
                permission_id: "perm_001".to_string(),
                permission: "user:read".to_string(),
                resource_type: "user".to_string(),
                granted_at: chrono::Utc::now() - chrono::Duration::hours(1),
                granted_by: "admin_001".to_string(),
                expires_at: None,
            },
            UserPermission {
                permission_id: "perm_002".to_string(),
                permission: "profile:update".to_string(),
                resource_type: "profile".to_string(),
                granted_at: chrono::Utc::now() - chrono::Duration::days(1),
                granted_by: "system".to_string(),
                expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
            },
        ];

        let total_count = permissions.len() as u64;
        let response = UserPermissionListResponse {
            user_id: request.user_id,
            permissions,
            total_count,
            page_token: request.page_token,
            has_more: false,
        };

        info!(
            "用户权限列表获取成功: user_id={}, count={}",
            response.user_id, response.total_count
        );
        Ok(response)
    }

    async fn batch_check_permissions(
        &self,
        request: BatchPermissionCheckRequest,
    ) -> SecurityResult<BatchPermissionCheckResponse> {
        let context = ErrorContext::new("batch_check_permissions").with_user_id(&request.user_id);

        debug!(
            "批量检查权限: user_id={}, count={}",
            request.user_id,
            request.permissions.len()
        );

        let mut results = Vec::new();
        for permission in &request.permissions {
            // 简单的权限检查逻辑
            let allowed = permission.contains("read") || permission.contains("profile");

            results.push(PermissionCheckResult {
                permission: permission.clone(),
                allowed,
                reason: if allowed {
                    Some("权限验证通过".to_string())
                } else {
                    Some("权限不足".to_string())
                },
            });
        }

        let response = BatchPermissionCheckResponse {
            user_id: request.user_id,
            results,
            evaluated_at: chrono::Utc::now(),
        };

        info!(
            "批量权限检查完成: user_id={}, count={}",
            response.user_id,
            response.results.len()
        );
        Ok(response)
    }

    async fn create_role(&self, request: CreateRoleRequest) -> SecurityResult<CreateRoleResponse> {
        let context = ErrorContext::new("create_role")
            .with_user_id(&request.creator_id)
            .with_extra("role_name", &request.role_name);

        debug!(
            "创建角色: name={}, creator={}",
            request.role_name, request.creator_id
        );

        // 验证角色名称
        self.validate_role_name(&request.role_name)?;

        let role_id = uuid::Uuid::new_v4().to_string();

        let response = CreateRoleResponse {
            role_id: role_id.clone(),
            role_name: request.role_name,
            creator_id: request.creator_id,
            created_at: chrono::Utc::now(),
            status: "active".to_string(),
        };

        info!(
            "角色创建成功: role_id={}, name={}",
            role_id, response.role_name
        );
        Ok(response)
    }

    async fn delete_role(&self, request: DeleteRoleRequest) -> SecurityResult<DeleteRoleResponse> {
        let context = ErrorContext::new("delete_role")
            .with_user_id(&request.operator_id)
            .with_extra("role_id", &request.role_id);

        debug!(
            "删除角色: role_id={}, operator={}",
            request.role_id, request.operator_id
        );

        // 验证角色ID
        if request.role_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_id".to_string(),
                reason: "角色ID不能为空".to_string(),
            });
        }

        let response = DeleteRoleResponse {
            role_id: request.role_id.clone(),
            deleted: true,
            operator_id: request.operator_id,
            deleted_at: chrono::Utc::now(),
        };

        info!("角色删除成功: role_id={}", response.role_id);
        Ok(response)
    }

    async fn update_role(&self, request: UpdateRoleRequest) -> SecurityResult<UpdateRoleResponse> {
        let context = ErrorContext::new("update_role")
            .with_user_id(&request.operator_id)
            .with_extra("role_id", &request.role_id);

        debug!(
            "更新角色: role_id={}, operator={}",
            request.role_id, request.operator_id
        );

        // 验证角色ID
        if request.role_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_id".to_string(),
                reason: "角色ID不能为空".to_string(),
            });
        }

        let response = UpdateRoleResponse {
            role_id: request.role_id.clone(),
            updated: true,
            operator_id: request.operator_id,
            updated_at: chrono::Utc::now(),
        };

        info!("角色更新成功: role_id={}", response.role_id);
        Ok(response)
    }

    async fn list_roles(&self, request: ListRolesRequest) -> SecurityResult<ListRolesResponse> {
        let context = ErrorContext::new("list_roles");

        debug!("获取角色列表: page_size={:?}", request.page_size);

        // 模拟角色列表
        let roles = vec![
            Role {
                role_id: "role_001".to_string(),
                role_name: "管理员".to_string(),
                description: Some("系统管理员角色".to_string()),
                permissions: vec!["admin:*".to_string(), "user:*".to_string()],
                created_at: chrono::Utc::now() - chrono::Duration::days(30),
                updated_at: chrono::Utc::now() - chrono::Duration::hours(1),
                status: "active".to_string(),
            },
            Role {
                role_id: "role_002".to_string(),
                role_name: "普通用户".to_string(),
                description: Some("标准用户角色".to_string()),
                permissions: vec!["profile:read".to_string(), "profile:update".to_string()],
                created_at: chrono::Utc::now() - chrono::Duration::days(20),
                updated_at: chrono::Utc::now() - chrono::Duration::hours(2),
                status: "active".to_string(),
            },
        ];

        let response = ListRolesResponse {
            roles,
            total_count: 2,
            page_token: request.page_token,
            has_more: false,
        };

        info!("角色列表获取成功: count={}", response.total_count);
        Ok(response)
    }

    async fn assign_role(&self, request: AssignRoleRequest) -> SecurityResult<AssignRoleResponse> {
        let context = ErrorContext::new("assign_role")
            .with_user_id(&request.operator_id)
            .with_extra("target_user_id", &request.target_user_id)
            .with_extra("role_id", &request.role_id);

        debug!(
            "分配角色: operator={}, target={}, role={}",
            request.operator_id, request.target_user_id, request.role_id
        );

        // 验证角色ID和用户ID
        if request.role_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "role_id".to_string(),
                reason: "角色ID不能为空".to_string(),
            });
        }

        if request.target_user_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "target_user_id".to_string(),
                reason: "目标用户ID不能为空".to_string(),
            });
        }

        let response = AssignRoleResponse {
            assigned: true,
            assignment_id: uuid::Uuid::new_v4().to_string(),
            operator_id: request.operator_id,
            target_user_id: request.target_user_id,
            role_id: request.role_id,
            assigned_at: chrono::Utc::now(),
        };

        info!(
            "角色分配成功: assignment_id={}, target={}, role={}",
            response.assignment_id, response.target_user_id, response.role_id
        );
        Ok(response)
    }
}

// 数据结构定义

/// 权限检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckRequest {
    /// 用户ID
    pub user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 资源标识符
    pub resource: String,
    /// 上下文信息
    pub context: Option<HashMap<String, String>>,
}

/// 权限检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    /// 是否有权限
    pub has_permission: bool,
    /// 用户ID
    pub user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 资源标识符
    pub resource: String,
    /// 原因说明
    pub reason: Option<String>,
    /// 评估时间
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
}

/// 创建策略请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    /// 策略名称
    pub policy_name: String,
    /// 策略描述
    pub description: Option<String>,
    /// 策略规则
    pub rules: Vec<PolicyRule>,
    /// 创建者ID
    pub creator_id: String,
    /// 策略类型
    pub policy_type: PolicyType,
}

/// 创建策略响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyResponse {
    /// 策略ID
    pub policy_id: String,
    /// 策略名称
    pub policy_name: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 策略状态
    pub status: String,
}

/// 删除策略请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyRequest {
    /// 策略ID
    pub policy_id: String,
    /// 操作者ID
    pub operator_id: String,
}

/// 删除策略响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyResponse {
    /// 策略ID
    pub policy_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 操作者ID
    pub operator_id: String,
    /// 删除时间
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

/// 更新策略请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicyRequest {
    /// 策略ID
    pub policy_id: String,
    /// 策略名称
    pub policy_name: Option<String>,
    /// 策略描述
    pub description: Option<String>,
    /// 策略规则
    pub rules: Option<Vec<PolicyRule>>,
    /// 操作者ID
    pub operator_id: String,
}

/// 更新策略响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicyResponse {
    /// 策略ID
    pub policy_id: String,
    /// 是否更新成功
    pub updated: bool,
    /// 操作者ID
    pub operator_id: String,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 策略评估请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationRequest {
    /// 用户ID
    pub user_id: String,
    /// 资源标识符
    pub resource: String,
    /// 操作类型
    pub action: String,
    /// 上下文信息
    pub context: Option<HashMap<String, String>>,
    /// 要评估的策略ID列表
    pub policy_ids: Option<Vec<String>>,
}

/// 策略评估响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResponse {
    /// 是否允许
    pub allowed: bool,
    /// 用户ID
    pub user_id: String,
    /// 资源标识符
    pub resource: String,
    /// 操作类型
    pub action: String,
    /// 评估的策略列表
    pub evaluated_policies: Vec<String>,
    /// 决策原因
    pub reason: Option<String>,
    /// 评估时间
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
}

/// 授权权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissionRequest {
    /// 授权者ID
    pub granter_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 资源类型
    pub resource_type: Option<String>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 授权原因
    pub reason: Option<String>,
}

/// 授权权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissionResponse {
    /// 是否授权成功
    pub granted: bool,
    /// 权限ID
    pub permission_id: String,
    /// 授权者ID
    pub granter_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 授权时间
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

/// 撤销权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionRequest {
    /// 撤销者ID
    pub revoker_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 撤销原因
    pub reason: Option<String>,
}

/// 撤销权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionResponse {
    /// 是否撤销成功
    pub revoked: bool,
    /// 撤销者ID
    pub revoker_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 权限标识符
    pub permission: String,
    /// 撤销时间
    pub revoked_at: chrono::DateTime<chrono::Utc>,
}

/// 用户权限列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionListRequest {
    /// 用户ID
    pub user_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页令牌
    pub page_token: Option<String>,
}

/// 用户权限列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionListResponse {
    /// 用户ID
    pub user_id: String,
    /// 权限列表
    pub permissions: Vec<UserPermission>,
    /// 总数
    pub total_count: u64,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
}

/// 批量权限检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPermissionCheckRequest {
    /// 用户ID
    pub user_id: String,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 资源标识符
    pub resource: Option<String>,
    /// 上下文信息
    pub context: Option<HashMap<String, String>>,
}

/// 批量权限检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPermissionCheckResponse {
    /// 用户ID
    pub user_id: String,
    /// 检查结果
    pub results: Vec<PermissionCheckResult>,
    /// 评估时间
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
}

/// 创建角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    /// 角色名称
    pub role_name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
    /// 创建者ID
    pub creator_id: String,
    /// 角色类型
    pub role_type: Option<RoleType>,
}

/// 创建角色响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 角色状态
    pub status: String,
}

/// 删除角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleRequest {
    /// 角色ID
    pub role_id: String,
    /// 操作者ID
    pub operator_id: String,
}

/// 删除角色响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleResponse {
    /// 角色ID
    pub role_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 操作者ID
    pub operator_id: String,
    /// 删除时间
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

/// 更新角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: Option<String>,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Option<Vec<String>>,
    /// 操作者ID
    pub operator_id: String,
}

/// 更新角色响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleResponse {
    /// 角色ID
    pub role_id: String,
    /// 是否更新成功
    pub updated: bool,
    /// 操作者ID
    pub operator_id: String,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 角色列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 角色类型过滤
    pub role_type: Option<RoleType>,
    /// 状态过滤
    pub status: Option<String>,
}

/// 角色列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesResponse {
    /// 角色列表
    pub roles: Vec<Role>,
    /// 总数
    pub total_count: u64,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
}

/// 分配角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    /// 操作者ID
    pub operator_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 角色ID
    pub role_id: String,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 分配原因
    pub reason: Option<String>,
}

/// 分配角色响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleResponse {
    /// 是否分配成功
    pub assigned: bool,
    /// 分配ID
    pub assignment_id: String,
    /// 操作者ID
    pub operator_id: String,
    /// 目标用户ID
    pub target_user_id: String,
    /// 角色ID
    pub role_id: String,
    /// 分配时间
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

// 辅助数据结构

/// 用户权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermission {
    /// 权限ID
    pub permission_id: String,
    /// 权限标识符
    pub permission: String,
    /// 资源类型
    pub resource_type: String,
    /// 授权时间
    pub granted_at: chrono::DateTime<chrono::Utc>,
    /// 授权者
    pub granted_by: String,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 权限检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResult {
    /// 权限标识符
    pub permission: String,
    /// 是否允许
    pub allowed: bool,
    /// 原因说明
    pub reason: Option<String>,
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 角色状态
    pub status: String,
}

/// 策略规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 规则条件
    pub conditions: Vec<PolicyCondition>,
    /// 规则动作
    pub effect: PolicyEffect,
    /// 规则优先级
    pub priority: u32,
}

/// 策略条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    /// 条件类型
    pub condition_type: String,
    /// 条件值
    pub value: serde_json::Value,
    /// 比较操作符
    pub operator: String,
}

/// 策略效果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
    /// 允许
    Allow,
    /// 拒绝
    Deny,
}

/// 策略类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    /// 基于角色的访问控制
    RBAC,
    /// 基于属性的访问控制
    ABAC,
    /// 混合策略
    Hybrid,
}

/// 角色类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoleType {
    /// 系统角色
    System,
    /// 业务角色
    Business,
    /// 自定义角色
    Custom,
}
