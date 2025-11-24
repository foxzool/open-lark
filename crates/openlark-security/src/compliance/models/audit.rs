//! 审计日志数据模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 审计日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditLevel {
    /// 信息级别
    #[serde(rename = "info")]
    Info,
    /// 警告级别
    #[serde(rename = "warning")]
    Warning,
    /// 错误级别
    #[serde(rename = "error")]
    Error,
    /// 严重级别
    #[serde(rename = "critical")]
    Critical,
}

/// 审计日志类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditType {
    /// 用户操作
    #[serde(rename = "user_action")]
    UserAction,
    /// 系统事件
    #[serde(rename = "system_event")]
    SystemEvent,
    /// 安全事件
    #[serde(rename = "security_event")]
    SecurityEvent,
    /// 数据访问
    #[serde(rename = "data_access")]
    DataAccess,
    /// 权限变更
    #[serde(rename = "permission_change")]
    PermissionChange,
    /// 配置变更
    #[serde(rename = "config_change")]
    ConfigChange,
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// 日志ID
    pub log_id: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 级别
    pub level: AuditLevel,
    /// 类型
    pub audit_type: AuditType,
    /// 用户ID
    pub user_id: Option<String>,
    /// 操作
    pub action: String,
    /// 资源
    pub resource: Option<String>,
    /// 资源类型
    pub resource_type: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 结果
    pub result: AuditResult,
    /// 详情
    pub details: Option<serde_json::Value>,
    /// 会话ID
    pub session_id: Option<String>,
    /// 追踪ID
    pub trace_id: Option<String>,
}

/// 审计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    /// 是否成功
    pub success: bool,
    /// 错误代码
    pub error_code: Option<String>,
    /// 错误消息
    pub error_message: Option<String>,
    /// 响应时间（毫秒）
    pub response_time_ms: Option<u64>,
}

/// 审计日志查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQueryRequest {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 审计类型列表
    pub audit_types: Option<Vec<AuditType>>,
    /// 级别列表
    pub levels: Option<Vec<AuditLevel>>,
    /// 操作列表
    pub actions: Option<Vec<String>>,
    /// 资源类型
    pub resource_types: Option<Vec<String>>,
    /// IP地址
    pub ip_addresses: Option<Vec<String>>,
    /// 分页
    pub page: Option<u32>,
    /// 页面大小
    pub page_size: Option<u32>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    /// 升序
    #[serde(rename = "asc")]
    Asc,
    /// 降序
    #[serde(rename = "desc")]
    Desc,
}

/// 审计日志查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQueryResponse {
    /// 日志列表
    pub logs: Vec<AuditLog>,
    /// 总数量
    pub total_count: u32,
    /// 当前页
    pub current_page: u32,
    /// 页面大小
    pub page_size: u32,
    /// 总页数
    pub total_pages: u32,
}

/// 审计日志创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogCreateRequest {
    /// 级别
    pub level: AuditLevel,
    /// 类型
    pub audit_type: AuditType,
    /// 用户ID
    pub user_id: Option<String>,
    /// 操作
    pub action: String,
    /// 资源
    pub resource: Option<String>,
    /// 资源类型
    pub resource_type: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 结果
    pub result: AuditResult,
    /// 详情
    pub details: Option<serde_json::Value>,
    /// 会话ID
    pub session_id: Option<String>,
    /// 追踪ID
    pub trace_id: Option<String>,
}

/// 审计日志创建响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogCreateResponse {
    /// 是否成功
    pub success: bool,
    /// 日志ID
    pub log_id: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 消息
    pub message: String,
}
