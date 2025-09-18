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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_user_id_type_enum() {
        assert_eq!(
            serde_json::to_string(&UserIdType::OpenId).unwrap(),
            "\"open_id\""
        );
        assert_eq!(
            serde_json::to_string(&UserIdType::UserId).unwrap(),
            "\"user_id\""
        );
        assert_eq!(
            serde_json::to_string(&UserIdType::UnionId).unwrap(),
            "\"union_id\""
        );
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
    }

    #[test]
    fn test_department_id_type_enum() {
        assert_eq!(
            serde_json::to_string(&DepartmentIdType::OpenDepartmentId).unwrap(),
            "\"open_department_id\""
        );
        assert_eq!(
            serde_json::to_string(&DepartmentIdType::DepartmentId).unwrap(),
            "\"department_id\""
        );
    }

    #[test]
    fn test_department_id_type_as_str() {
        assert_eq!(
            DepartmentIdType::OpenDepartmentId.as_str(),
            "open_department_id"
        );
        assert_eq!(DepartmentIdType::DepartmentId.as_str(), "department_id");
    }

    #[test]
    fn test_application_full() {
        let app = Application {
            app_id: Some("app123".to_string()),
            app_name: Some("测试应用".to_string()),
            description: Some("这是一个测试应用".to_string()),
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            app_type: Some(AppType::SelfBuilt),
            status: Some(AppStatus::Published),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T12:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains("app123"));
        assert!(json.contains("测试应用"));
        assert!(json.contains("self_built"));
        assert!(json.contains("published"));
    }

    #[test]
    fn test_app_type_enum() {
        assert_eq!(
            serde_json::to_string(&AppType::SelfBuilt).unwrap(),
            "\"self_built\""
        );
        assert_eq!(
            serde_json::to_string(&AppType::Marketplace).unwrap(),
            "\"marketplace\""
        );
    }

    #[test]
    fn test_app_status_enum() {
        assert_eq!(
            serde_json::to_string(&AppStatus::Developing).unwrap(),
            "\"developing\""
        );
        assert_eq!(
            serde_json::to_string(&AppStatus::Published).unwrap(),
            "\"published\""
        );
        assert_eq!(
            serde_json::to_string(&AppStatus::Removed).unwrap(),
            "\"removed\""
        );
        assert_eq!(
            serde_json::to_string(&AppStatus::Disabled).unwrap(),
            "\"disabled\""
        );
    }

    #[test]
    fn test_app_version() {
        let version = AppVersion {
            version_id: Some("v123".to_string()),
            version: Some("1.2.0".to_string()),
            version_name: Some("重要更新".to_string()),
            remark: Some("修复了若干bug".to_string()),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            publish_time: Some("2024-01-02T00:00:00Z".to_string()),
            status: Some(VersionStatus::Published),
        };
        let json = serde_json::to_string(&version).unwrap();
        assert!(json.contains("v123"));
        assert!(json.contains("1.2.0"));
        assert!(json.contains("重要更新"));
        assert!(json.contains("published"));
    }

    #[test]
    fn test_version_status_enum() {
        assert_eq!(
            serde_json::to_string(&VersionStatus::Developing).unwrap(),
            "\"developing\""
        );
        assert_eq!(
            serde_json::to_string(&VersionStatus::Auditing).unwrap(),
            "\"auditing\""
        );
        assert_eq!(
            serde_json::to_string(&VersionStatus::Published).unwrap(),
            "\"published\""
        );
        assert_eq!(
            serde_json::to_string(&VersionStatus::Rejected).unwrap(),
            "\"rejected\""
        );
    }

    #[test]
    fn test_app_collaborator() {
        let collaborator = AppCollaborator {
            collaborator_id: Some("user123".to_string()),
            collaborator_type: Some(CollaboratorType::User),
            permissions: Some(vec!["read".to_string(), "write".to_string()]),
        };
        let json = serde_json::to_string(&collaborator).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("user"));
        assert!(json.contains("read"));
    }

    #[test]
    fn test_collaborator_type_enum() {
        assert_eq!(
            serde_json::to_string(&CollaboratorType::User).unwrap(),
            "\"user\""
        );
        assert_eq!(
            serde_json::to_string(&CollaboratorType::Group).unwrap(),
            "\"group\""
        );
    }

    #[test]
    fn test_permission_scope() {
        let scope = PermissionScope {
            permission: Some("contacts:read".to_string()),
            granted: Some(true),
            status: Some(AuthStatus::Granted),
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("contacts:read"));
        assert!(json.contains("true"));
        assert!(json.contains("granted"));
    }

    #[test]
    fn test_auth_status_enum() {
        assert_eq!(
            serde_json::to_string(&AuthStatus::Granted).unwrap(),
            "\"granted\""
        );
        assert_eq!(
            serde_json::to_string(&AuthStatus::NotGranted).unwrap(),
            "\"not_granted\""
        );
        assert_eq!(
            serde_json::to_string(&AuthStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&AuthStatus::Rejected).unwrap(),
            "\"rejected\""
        );
    }

    #[test]
    fn test_app_admin() {
        let admin = AppAdmin {
            user_id: Some("admin123".to_string()),
            permissions: Some(vec![
                AdminPermission::AppManagement,
                AdminPermission::UserManagement,
            ]),
        };
        let json = serde_json::to_string(&admin).unwrap();
        assert!(json.contains("admin123"));
        assert!(json.contains("app_management"));
        assert!(json.contains("user_management"));
    }

    #[test]
    fn test_admin_permission_enum() {
        assert_eq!(
            serde_json::to_string(&AdminPermission::AppManagement).unwrap(),
            "\"app_management\""
        );
        assert_eq!(
            serde_json::to_string(&AdminPermission::UserManagement).unwrap(),
            "\"user_management\""
        );
        assert_eq!(
            serde_json::to_string(&AdminPermission::PermissionManagement).unwrap(),
            "\"permission_management\""
        );
    }

    #[test]
    fn test_app_availability() {
        let availability = AppAvailability {
            is_visible_to_all: Some(false),
            visible_list: Some(VisibilityList {
                user_list: Some(vec!["user1".to_string(), "user2".to_string()]),
                department_list: Some(vec!["dept1".to_string()]),
                group_list: Some(vec!["group1".to_string()]),
            }),
            invisible_list: None,
        };
        let json = serde_json::to_string(&availability).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("user1"));
        assert!(json.contains("dept1"));
    }

    #[test]
    fn test_visibility_list() {
        let list = VisibilityList {
            user_list: Some(vec!["user123".to_string()]),
            department_list: Some(vec!["tech".to_string(), "product".to_string()]),
            group_list: None,
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("user123"));
        assert!(json.contains("tech"));
        assert!(json.contains("product"));
    }

    #[test]
    fn test_app_usage() {
        let usage = AppUsage {
            date: Some("2024-01-01".to_string()),
            active_users: Some(150),
            new_users: Some(20),
            message_push_count: Some(500),
        };
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("150"));
        assert!(json.contains("20"));
        assert!(json.contains("500"));
    }

    #[test]
    fn test_department_usage() {
        let usage = DepartmentUsage {
            department_id: Some("dept123".to_string()),
            department_name: Some("技术部".to_string()),
            active_users: Some(25),
            total_users: Some(30),
        };
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("dept123"));
        assert!(json.contains("技术部"));
        assert!(json.contains("25"));
        assert!(json.contains("30"));
    }

    #[test]
    fn test_app_feedback() {
        let feedback = AppFeedback {
            feedback_id: Some("fb123".to_string()),
            user_id: Some("user456".to_string()),
            feedback_type: Some(FeedbackType::Bug),
            content: Some("应用启动很慢".to_string()),
            rating: Some(3),
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            status: Some(FeedbackStatus::Processing),
        };
        let json = serde_json::to_string(&feedback).unwrap();
        assert!(json.contains("fb123"));
        assert!(json.contains("bug"));
        assert!(json.contains("应用启动很慢"));
        assert!(json.contains("processing"));
    }

    #[test]
    fn test_feedback_type_enum() {
        assert_eq!(
            serde_json::to_string(&FeedbackType::Bug).unwrap(),
            "\"bug\""
        );
        assert_eq!(
            serde_json::to_string(&FeedbackType::Feature).unwrap(),
            "\"feature\""
        );
        assert_eq!(
            serde_json::to_string(&FeedbackType::Other).unwrap(),
            "\"other\""
        );
    }

    #[test]
    fn test_feedback_status_enum() {
        assert_eq!(
            serde_json::to_string(&FeedbackStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&FeedbackStatus::Processing).unwrap(),
            "\"processing\""
        );
        assert_eq!(
            serde_json::to_string(&FeedbackStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&FeedbackStatus::Closed).unwrap(),
            "\"closed\""
        );
    }

    #[test]
    fn test_app_badge() {
        let badge = AppBadge {
            badge_type: Some(BadgeType::Number),
            content: Some("5".to_string()),
        };
        let json = serde_json::to_string(&badge).unwrap();
        assert!(json.contains("number"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_badge_type_enum() {
        assert_eq!(
            serde_json::to_string(&BadgeType::Number).unwrap(),
            "\"number\""
        );
        assert_eq!(serde_json::to_string(&BadgeType::Dot).unwrap(), "\"dot\"");
        assert_eq!(
            serde_json::to_string(&BadgeType::Clear).unwrap(),
            "\"clear\""
        );
    }

    #[test]
    fn test_order() {
        let order = Order {
            order_id: Some("order123".to_string()),
            status: Some(OrderStatus::Paid),
            pricing_plan_id: Some("plan456".to_string()),
            quantity: Some(10),
            purchase_time: Some("2024-01-01T00:00:00Z".to_string()),
            expire_time: Some("2025-01-01T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("order123"));
        assert!(json.contains("paid"));
        assert!(json.contains("plan456"));
        assert!(json.contains("10"));
    }

    #[test]
    fn test_order_status_enum() {
        assert_eq!(
            serde_json::to_string(&OrderStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Paid).unwrap(),
            "\"paid\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Cancelled).unwrap(),
            "\"cancelled\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Expired).unwrap(),
            "\"expired\""
        );
    }

    #[test]
    fn test_pricing_plan() {
        let plan = PricingPlan {
            pricing_plan_id: Some("plan123".to_string()),
            plan_name: Some("专业版".to_string()),
            description: Some("适合中型团队".to_string()),
            price: Some("99.00".to_string()),
            billing_cycle: Some(BillingCycle::Monthly),
        };
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("plan123"));
        assert!(json.contains("专业版"));
        assert!(json.contains("99.00"));
        assert!(json.contains("monthly"));
    }

    #[test]
    fn test_billing_cycle_enum() {
        assert_eq!(
            serde_json::to_string(&BillingCycle::Monthly).unwrap(),
            "\"monthly\""
        );
        assert_eq!(
            serde_json::to_string(&BillingCycle::Yearly).unwrap(),
            "\"yearly\""
        );
        assert_eq!(
            serde_json::to_string(&BillingCycle::Once).unwrap(),
            "\"once\""
        );
    }

    #[test]
    fn test_audit_status() {
        let status = AuditStatus {
            status: Some(AuditResult::Approved),
            comment: Some("应用符合规范".to_string()),
            audit_time: Some("2024-01-01T14:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("approved"));
        assert!(json.contains("应用符合规范"));
    }

    #[test]
    fn test_audit_result_enum() {
        assert_eq!(
            serde_json::to_string(&AuditResult::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&AuditResult::Approved).unwrap(),
            "\"approved\""
        );
        assert_eq!(
            serde_json::to_string(&AuditResult::Rejected).unwrap(),
            "\"rejected\""
        );
    }

    #[test]
    fn test_contacts_range() {
        let range = ContactsRange {
            range_type: Some(ContactsRangeType::Some),
            department_list: Some(vec!["dept1".to_string(), "dept2".to_string()]),
            user_list: Some(vec!["user1".to_string()]),
        };
        let json = serde_json::to_string(&range).unwrap();
        assert!(json.contains("some"));
        assert!(json.contains("dept1"));
        assert!(json.contains("user1"));
    }

    #[test]
    fn test_contacts_range_type_enum() {
        assert_eq!(
            serde_json::to_string(&ContactsRangeType::All).unwrap(),
            "\"all\""
        );
        assert_eq!(
            serde_json::to_string(&ContactsRangeType::Some).unwrap(),
            "\"some\""
        );
        assert_eq!(
            serde_json::to_string(&ContactsRangeType::AdminRange).unwrap(),
            "\"admin_range\""
        );
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_app = Application {
            app_id: Some("minimal".to_string()),
            app_name: Some("最小应用".to_string()),
            description: None,
            avatar_url: None,
            app_type: None,
            status: None,
            create_time: None,
            update_time: None,
        };
        let json = serde_json::to_string(&minimal_app).unwrap();
        assert!(json.contains("minimal"));
        assert!(json.contains("最小应用"));
        assert!(!json.contains("description"));
    }
}
