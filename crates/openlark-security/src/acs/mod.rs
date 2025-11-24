//! 访问控制模块
//!
//! 提供基于角色的访问控制（RBAC）和基于属性的访问控制（ABAC）功能。
//!
//! ## 功能特性
//!
//! - 角色权限管理
//! - 策略引擎评估
//! - 权限检查与授权
//! - 审计日志记录
//! - 细粒度权限控制

pub mod service;
pub mod v1;

// 重新导出主要类型
pub use service::{
    AccessControlService, AssignRoleRequest, AssignRoleResponse, BatchPermissionCheckRequest,
    BatchPermissionCheckResponse, CreatePolicyRequest, CreatePolicyResponse, CreateRoleRequest,
    CreateRoleResponse, DefaultAccessControlService, DeletePolicyRequest, DeletePolicyResponse,
    DeleteRoleRequest, DeleteRoleResponse, GrantPermissionRequest, GrantPermissionResponse,
    ListRolesRequest, ListRolesResponse, PermissionCheckRequest, PermissionCheckResponse,
    PolicyEffect, PolicyEvaluationRequest, PolicyEvaluationResponse, PolicyRule, PolicyType,
    RevokePermissionRequest, RevokePermissionResponse, Role, RoleType, UpdatePolicyRequest,
    UpdatePolicyResponse, UpdateRoleRequest, UpdateRoleResponse, UserPermission,
    UserPermissionListRequest, UserPermissionListResponse,
};

// API版本导出
#[cfg(feature = "v1")]
pub use v1::{
    create_assign_role_request, create_batch_permission_check_request,
    create_create_policy_request, create_create_role_request, create_delete_policy_request,
    create_delete_role_request, create_grant_permission_request, create_list_roles_request,
    create_permission_check_request, create_policy_evaluation_request, create_policy_rule,
    create_revoke_permission_request, create_update_policy_request, create_update_role_request,
    create_user_permission_list_request, AssignRoleRequestBuilder,
    BatchPermissionCheckRequestBuilder, CreatePolicyRequestBuilder, CreateRoleRequestBuilder,
    DeletePolicyRequestBuilder, DeleteRoleRequestBuilder, GrantPermissionRequestBuilder,
    ListRolesRequestBuilder, PermissionCheckRequestBuilder, PermissionV1API,
    PolicyEvaluationRequestBuilder, PolicyRuleBuilder, PolicyV1API, RevokePermissionRequestBuilder,
    RoleV1API, UpdatePolicyRequestBuilder, UpdateRoleRequestBuilder,
    UserPermissionListRequestBuilder,
};

#[cfg(any(feature = "v2", feature = "v3"))]
compile_error!("v2 and v3 API support not yet implemented");
