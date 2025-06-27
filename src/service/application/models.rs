use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIdType {
    #[serde(rename = "open_id")]
    OpenId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "union_id")]
    UnionId,
}

impl UserIdType {
    pub fn as_str(&self) -> &str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
        }
    }
}

/// 部门ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DepartmentIdType {
    #[serde(rename = "open_department_id")]
    OpenDepartmentId,
    #[serde(rename = "department_id")]
    DepartmentId,
}

impl DepartmentIdType {
    pub fn as_str(&self) -> &str {
        match self {
            DepartmentIdType::OpenDepartmentId => "open_department_id",
            DepartmentIdType::DepartmentId => "department_id",
        }
    }
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 应用类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<AppType>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AppStatus>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 应用类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppType {
    /// 自建应用
    #[serde(rename = "self_built")]
    SelfBuilt,
    /// 应用商店应用
    #[serde(rename = "marketplace")]
    Marketplace,
}

/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStatus {
    /// 开发中
    #[serde(rename = "developing")]
    Developing,
    /// 已发布
    #[serde(rename = "published")]
    Published,
    /// 已下架
    #[serde(rename = "removed")]
    Removed,
    /// 已停用
    #[serde(rename = "disabled")]
    Disabled,
}

/// 应用版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersion {
    /// 版本ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 版本名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    /// 版本描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 发布时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    /// 版本状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VersionStatus>,
}

/// 版本状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionStatus {
    /// 开发中
    #[serde(rename = "developing")]
    Developing,
    /// 审核中
    #[serde(rename = "auditing")]
    Auditing,
    /// 已发布
    #[serde(rename = "published")]
    Published,
    /// 审核拒绝
    #[serde(rename = "rejected")]
    Rejected,
}

/// 应用协作者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCollaborator {
    /// 协作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_id: Option<String>,
    /// 协作者类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_type: Option<CollaboratorType>,
    /// 权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// 协作者类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaboratorType {
    /// 用户
    #[serde(rename = "user")]
    User,
    /// 群组
    #[serde(rename = "group")]
    Group,
}

/// 权限范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionScope {
    /// 权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// 是否已授权
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted: Option<bool>,
    /// 授权状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AuthStatus>,
}

/// 授权状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthStatus {
    /// 已授权
    #[serde(rename = "granted")]
    Granted,
    /// 未授权
    #[serde(rename = "not_granted")]
    NotGranted,
    /// 审核中
    #[serde(rename = "pending")]
    Pending,
    /// 已拒绝
    #[serde(rename = "rejected")]
    Rejected,
}

/// 应用管理员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAdmin {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 管理权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<AdminPermission>>,
}

/// 管理员权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminPermission {
    /// 应用管理
    #[serde(rename = "app_management")]
    AppManagement,
    /// 用户管理
    #[serde(rename = "user_management")]
    UserManagement,
    /// 权限管理
    #[serde(rename = "permission_management")]
    PermissionManagement,
}

/// 应用可用范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAvailability {
    /// 是否对所有人可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible_to_all: Option<bool>,
    /// 白名单用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<VisibilityList>,
    /// 黑名单用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible_list: Option<VisibilityList>,
}

/// 可见性列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityList {
    /// 用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<String>>,
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<String>>,
    /// 用户组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<String>>,
}

/// 应用使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsage {
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 活跃用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i64>,
    /// 新增用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_users: Option<i64>,
    /// 消息推送数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_push_count: Option<i64>,
}

/// 部门使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentUsage {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 活跃用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i64>,
    /// 总用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_users: Option<i64>,
}

/// 应用反馈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppFeedback {
    /// 反馈ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 反馈类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_type: Option<FeedbackType>,
    /// 反馈内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FeedbackStatus>,
}

/// 反馈类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    /// 问题反馈
    #[serde(rename = "bug")]
    Bug,
    /// 功能建议
    #[serde(rename = "feature")]
    Feature,
    /// 其他
    #[serde(rename = "other")]
    Other,
}

/// 反馈状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackStatus {
    /// 待处理
    #[serde(rename = "pending")]
    Pending,
    /// 处理中
    #[serde(rename = "processing")]
    Processing,
    /// 已完成
    #[serde(rename = "completed")]
    Completed,
    /// 已关闭
    #[serde(rename = "closed")]
    Closed,
}

/// 应用红点设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBadge {
    /// 红点类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_type: Option<BadgeType>,
    /// 红点内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 红点类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BadgeType {
    /// 数字红点
    #[serde(rename = "number")]
    Number,
    /// 圆点红点
    #[serde(rename = "dot")]
    Dot,
    /// 清除红点
    #[serde(rename = "clear")]
    Clear,
}

/// 付费订单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 订单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// 付费方案ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_id: Option<String>,
    /// 购买数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// 购买时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<String>,
    /// 到期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
}

/// 订单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 待支付
    #[serde(rename = "pending")]
    Pending,
    /// 已支付
    #[serde(rename = "paid")]
    Paid,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
    /// 已过期
    #[serde(rename = "expired")]
    Expired,
}

/// 付费方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingPlan {
    /// 方案ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_id: Option<String>,
    /// 方案名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// 方案描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// 计费周期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<BillingCycle>,
}

/// 计费周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    /// 月付
    #[serde(rename = "monthly")]
    Monthly,
    /// 年付
    #[serde(rename = "yearly")]
    Yearly,
    /// 一次性
    #[serde(rename = "once")]
    Once,
}

/// 应用审核状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatus {
    /// 审核状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AuditResult>,
    /// 审核意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 审核时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_time: Option<String>,
}

/// 审核结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    /// 待审核
    #[serde(rename = "pending")]
    Pending,
    /// 审核通过
    #[serde(rename = "approved")]
    Approved,
    /// 审核拒绝
    #[serde(rename = "rejected")]
    Rejected,
}

/// 通讯录权限范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsRange {
    /// 权限范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_type: Option<ContactsRangeType>,
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<String>>,
    /// 用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<String>>,
}

/// 通讯录权限范围类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactsRangeType {
    /// 全员
    #[serde(rename = "all")]
    All,
    /// 部分员工
    #[serde(rename = "some")]
    Some,
    /// 当前应用管理范围
    #[serde(rename = "admin_range")]
    AdminRange,
}
