//! Trust Party 可信第三方服务端点常量定义

// ===== 协作组织管理 =====

/// 获取可见关联组织列表
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &str =
    "/open-apis/trust_party/v1/collaboration_organizations";

/// 获取关联组织的部门和成员信息 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";

/// 获取关联组织详情 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";

/// 获取关联组织成员详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {user_id})
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";

/// 获取关联组织部门详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {department_id})
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";

/// 获取关联组织双方共享成员范围 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";

/// 管理员获取所有关联组织列表
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/admin";

// ===== 可搜可见规则管理 =====

/// 可搜可见规则操作（创建/查询）
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &str =
    "/open-apis/trust_party/v1/searchable_visible_rules";

/// 可搜可见规则操作（更新/删除） (需要使用 EndpointBuilder::replace_param 替换 {rule_id})
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &str =
    "/open-apis/trust_party/v1/searchable_visible_rules/{rule_id}";
