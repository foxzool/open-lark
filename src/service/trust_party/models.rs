use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 关联组织信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationOrganization {
    /// 关联组织ID
    pub org_id: String,
    /// 关联组织名称
    pub org_name: String,
    /// 关联组织类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<String>,
    /// 关联组织状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 关联关系创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 关联关系更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// 关联组织域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 关联组织描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 关联组织部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationDepartment {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 部门层级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 部门成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 关联组织成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationUser {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// 所属部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 职位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 组织架构信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStructure {
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<CollaborationDepartment>>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<CollaborationUser>>,
}

/// 共享成员范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedMemberScope {
    /// 共享范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 共享的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 共享的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 共享范围描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 可搜可见规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchableVisibleRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 适用的组织ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// 规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RuleConfig>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 规则配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// 可见性设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// 搜索权限设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// 适用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
    /// 例外情况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<String>>,
}

/// 规则适用范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleScope {
    /// 适用的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 适用的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 适用的用户组ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}
