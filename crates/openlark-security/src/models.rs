//! 安全认证服务数据模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 用户认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户类型
    pub user_type: UserType,
    /// 认证方式
    pub auth_method: AuthMethod,
    /// 认证时间
    pub auth_time: DateTime<Utc>,
    /// 认证IP地址
    pub ip_address: String,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
}

/// 用户类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
    /// 管理员
    Admin,
    /// 普通用户
    User,
    /// 服务账号
    ServiceAccount,
    /// 应用
    Application,
}

/// 认证方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    /// 密码认证
    Password,
    /// OAuth认证
    OAuth,
    /// SAML认证
    SAML,
    /// LDAP认证
    LDAP,
    /// 多因子认证
    MFA,
    /// API Key认证
    APIKey,
    /// JWT认证
    JWT,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备ID
    pub device_id: String,
    /// 设备类型
    pub device_type: DeviceType,
    /// 设备名称
    pub device_name: String,
    /// 操作系统
    pub os: String,
    /// 浏览器信息
    pub browser: Option<String>,
    /// 设备指纹
    pub fingerprint: Option<String>,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    /// 桌面设备
    Desktop,
    /// 移动设备
    Mobile,
    /// 平板设备
    Tablet,
    /// 服务器
    Server,
    /// IoT设备
    IoT,
}

/// 访问控制策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    /// 策略ID
    pub policy_id: String,
    /// 策略名称
    pub policy_name: String,
    /// 策略描述
    pub description: Option<String>,
    /// 策略类型
    pub policy_type: PolicyType,
    /// 策略规则
    pub rules: Vec<PolicyRule>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 是否启用
    pub enabled: bool,
}

/// 策略类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyType {
    /// 基于IP的访问控制
    IPBased,
    /// 基于时间的访问控制
    TimeBased,
    /// 基于角色的访问控制
    RoleBased,
    /// 基于资源的访问控制
    ResourceBased,
    /// 基于属性的访问控制
    AttributeBased,
}

/// 策略规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// 规则ID
    pub rule_id: String,
    /// 规则名称
    pub rule_name: String,
    /// 规则条件
    pub conditions: Vec<RuleCondition>,
    /// 允许的动作
    pub allowed_actions: Vec<String>,
    /// 拒绝的动作
    pub denied_actions: Vec<String>,
    /// 优先级
    pub priority: i32,
}

/// 规则条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// 条件类型
    pub condition_type: ConditionType,
    /// 条件操作符
    pub operator: ConditionOperator,
    /// 条件值
    pub value: String,
    /// 是否反向条件
    pub negate: bool,
}

/// 条件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    /// IP地址
    IPAddress,
    /// 时间范围
    TimeRange,
    /// 用户角色
    UserRole,
    /// 资源类型
    ResourceType,
    /// 动作类型
    ActionType,
    /// 地理位置
    GeographicLocation,
    /// 设备类型
    DeviceType,
}

/// 条件操作符
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    /// 等于
    Equals,
    /// 不等于
    NotEquals,
    /// 包含
    Contains,
    /// 不包含
    NotContains,
    /// 在范围内
    InRange,
    /// 不在范围内
    NotInRange,
    /// 正则匹配
    RegexMatch,
    /// 大于
    GreaterThan,
    /// 小于
    LessThan,
}

/// 令牌信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// 令牌类型
    pub token_type: TokenType,
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 令牌 scopes
    pub scopes: Vec<String>,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
    /// 颁发时间
    pub issued_at: DateTime<Utc>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 权限列表
    pub permissions: Vec<String>,
}

/// 令牌类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    /// 访问令牌
    AccessToken,
    /// 刷新令牌
    RefreshToken,
    /// 临时令牌
    TemporaryToken,
    /// 应用令牌
    AppToken,
    /// 服务令牌
    ServiceToken,
}

/// 安全事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: SecurityEventType,
    /// 事件级别
    pub severity: EventSeverity,
    /// 事件时间
    pub event_time: DateTime<Utc>,
    /// 用户ID
    pub user_id: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
    /// 事件描述
    pub description: String,
    /// 事件详情
    pub details: serde_json::Value,
    /// 是否已处理
    pub processed: bool,
    /// 处理时间
    pub processed_time: Option<DateTime<Utc>>,
}

/// 安全事件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    /// 登录成功
    LoginSuccess,
    /// 登录失败
    LoginFailed,
    /// 登出
    Logout,
    /// 密码修改
    PasswordChanged,
    /// 权限变更
    PermissionChanged,
    /// 可疑登录
    SuspiciousLogin,
    /// 账户锁定
    AccountLocked,
    /// 账户解锁
    AccountUnlocked,
    /// 策略违反
    PolicyViolation,
    /// 令牌过期
    TokenExpired,
    /// 令牌刷新
    TokenRefreshed,
    /// 访问被拒绝
    AccessDenied,
    /// 资源访问
    ResourceAccess,
}

/// 事件严重级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventSeverity {
    /// 低
    Low,
    /// 中
    Medium,
    /// 高
    High,
    /// 严重
    Critical,
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 会话ID
    pub session_id: String,
    /// 用户ID
    pub user_id: String,
    /// 应用ID
    pub app_id: Option<String>,
    /// 会话开始时间
    pub start_time: DateTime<Utc>,
    /// 会话过期时间
    pub expires_at: DateTime<Utc>,
    /// 最后活动时间
    pub last_activity: DateTime<Utc>,
    /// IP地址
    pub ip_address: String,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
    /// 会话状态
    pub status: SessionStatus,
    /// 权限列表
    pub permissions: Vec<String>,
}

/// 会话状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    /// 活跃
    Active,
    /// 已过期
    Expired,
    /// 已终止
    Terminated,
    /// 暂停
    Suspended,
}

/// 审计日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// 日志ID
    pub log_id: String,
    /// 操作时间
    pub timestamp: DateTime<Utc>,
    /// 操作类型
    pub action: String,
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户名
    pub username: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 操作结果
    pub result: ActionResult,
    /// 错误信息
    pub error_message: Option<String>,
    /// 请求详情
    pub request_details: Option<serde_json::Value>,
    /// 响应详情
    pub response_details: Option<serde_json::Value>,
}

/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionResult {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 部分成功
    PartialSuccess,
    /// 超时
    Timeout,
}
