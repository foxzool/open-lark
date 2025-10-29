// app_engine模块的数据模型定义

use serde::{Deserialize, Serialize};
/// 应用基本信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct AppInfo {
    /// 应用ID
#[serde(rename = "app_id")],
    pub app_id: String,
    /// 应用名称
    pub name: String,
    /// 应用描述
    pub description: Option<String>,
    /// 应用类型
#[serde(rename = "app_type")],
    pub app_type: Option<String>,
    /// 应用状态
#[serde(rename = "app_status")],
    pub app_status: AppStatus,
    /// 创建时间
#[serde(rename = "create_time")],
    pub create_time: Option<String>,
    /// 更新时间
#[serde(rename = "update_time")],
    pub update_time: Option<String>,
    /// 应用版本
    pub version: Option<String>,
    /// 命名空间
    pub namespace: Option<String>,
}
/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum AppStatus {,
/// 开发中
    Developing,
    /// 已发布
    Published,
    /// 已下线
    Offline,
    /// 已禁用
    Disabled,
}
/// 席位分配信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct SeatAssignment {
    /// 席位ID
#[serde(rename = "seat_id")],
    pub seat_id: String,
    /// 用户ID
#[serde(rename = "user_id")],
    pub user_id: String,
    /// 用户名
#[serde(rename = "user_name")],
    pub user_name: Option<String>,
    /// 席位类型
#[serde(rename = "seat_type")],
    pub seat_type: SeatType,
    /// 分配时间
#[serde(rename = "assign_time")],
    pub assign_time: String,
    /// 过期时间
#[serde(rename = "expire_time")],
    pub expire_time: Option<String>,
    /// 席位状态
#[serde(rename = "seat_status")],
    pub seat_status: SeatStatus,
}
/// 席位类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum SeatType {,
/// 开发者席位
    Developer,
    /// 用户席位
    User,
    /// 管理员席位
    Admin,
}
/// 席位状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum SeatStatus {,
/// 活跃
    Active,
    /// 已过期
    Expired,
    /// 已禁用
    Disabled,
}
/// 席位活跃信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct SeatActivity {
    /// 席位ID
#[serde(rename = "seat_id")],
    pub seat_id: String,
    /// 用户ID
#[serde(rename = "user_id")],
    pub user_id: String,
    /// 活跃度指标
#[serde(rename = "activity_metrics")],
    pub activity_metrics: ActivityMetrics,
    /// 最后活跃时间
#[serde(rename = "last_active_time")],
    pub last_active_time: Option<String>,
    /// 统计周期
#[serde(rename = "period")],
    pub period: String,
}
/// 活跃度指标
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct ActivityMetrics {
    /// 登录次数
#[serde(rename = "login_count")],
    pub login_count: i32,
    /// 操作次数
#[serde(rename = "operation_count")],
    pub operation_count: i32,
    /// 活跃时长（分钟）
#[serde(rename = "active_duration_minutes")],
    pub active_duration_minutes: i32,
}
/// 审计日志
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct AuditLog {
    /// 日志ID
#[serde(rename = "log_id")],
    pub log_id: String,
    /// 操作类型
#[serde(rename = "operation_type")],
    pub operation_type: String,
    /// 操作时间
#[serde(rename = "operation_time")],
    pub operation_time: String,
    /// 操作人
#[serde(rename = "operator")],
    pub operator: OperatorInfo,
    /// 操作描述
    pub description: Option<String>,
    /// 操作对象
#[serde(rename = "target_object")],
    pub target_object: TargetObject,
    /// 操作结果
#[serde(rename = "operation_result")],
    pub operation_result: OperationResult,
    /// IP地址
#[serde(rename = "ip_address")],
    pub ip_address: Option<String>,
}
/// 操作人信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct OperatorInfo {
    /// 用户ID
#[serde(rename = "user_id")],
    pub user_id: String,
    /// 用户名
#[serde(rename = "user_name")],
    pub user_name: Option<String>,
    /// 用户类型
#[serde(rename = "user_type")],
    pub user_type: Option<String>,
}
/// 操作对象
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct TargetObject {
    /// 对象类型
#[serde(rename = "object_type")],
    pub object_type: String,
    /// 对象ID
#[serde(rename = "object_id")],
    pub object_id: String,
    /// 对象名称
#[serde(rename = "object_name")],
    pub object_name: Option<String>,
}
/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum OperationResult {,
/// 成功
    Success,
    /// 失败
    Failed,
    /// 部分成功
    PartialSuccess,
}
/// 数据变更日志
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct DataChangeLog {
    /// 变更日志ID
#[serde(rename = "change_id")],
    pub change_id: String,
    /// 变更时间
#[serde(rename = "change_time")],
    pub change_time: String,
    /// 变更类型
#[serde(rename = "change_type")],
    pub change_type: ChangeType,
    /// 变更人
#[serde(rename = "changer")],
    pub changer: OperatorInfo,
    /// 变更对象
#[serde(rename = "target_record")],
    pub target_record: TargetRecord,
    /// 变更详情
#[serde(rename = "change_details")],
    pub change_details: Vec<ChangeDetail>,
}
/// 变更类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum ChangeType {,
/// 创建
    Create,
    /// 更新
    Update,
    /// 删除
    Delete,
}
/// 目标记录
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct TargetRecord {
    /// 记录类型
#[serde(rename = "record_type")],
    pub record_type: String,
    /// 记录ID
#[serde(rename = "record_id")],
    pub record_id: String,
    /// 记录名称
#[serde(rename = "record_name")],
    pub record_name: Option<String>,
}
/// 变更详情
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct ChangeDetail {
    /// 字段名
#[serde(rename = "field_name")],
    pub field_name: String,
    /// 变更前值
#[serde(rename = "old_value")],
    pub old_value: Option<serde_json::Value>,
    /// 变更后值
#[serde(rename = "new_value")],
    pub new_value: Option<serde_json::Value>,
    /// 变更类型
#[serde(rename = "change_type")],
    pub change_type: String,
}
/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct PermissionInfo {
    /// 权限ID
#[serde(rename = "permission_id")],
    pub permission_id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: Option<String>,
    /// 权限类型
#[serde(rename = "permission_type")],
    pub permission_type: PermissionType,
    /// 资源类型
#[serde(rename = "resource_type")],
    pub resource_type: String,
    /// 操作类型
#[serde(rename = "action_type")],
    pub action_type: String,
}
/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum PermissionType {,
/// 角色权限
    Role,
    /// 记录权限
    Record,
    /// 系统权限
    System,
}
/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct AppEngineResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}
/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct PaginatedAppEngineResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedAppEngineData<T>>,
}
/// 分页数据
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct PaginatedAppEngineData<T> {,
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}