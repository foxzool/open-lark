//! 通用数据模型
//!
//! 提供跨项目使用的通用数据结构。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 通用键值对结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    /// 键
    pub key: String,
    /// 值
    pub value: String,
}

/// 通用扩展信息
pub type ExtensionMap = HashMap<String, serde_json::Value>;

/// 设备基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBase {
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    pub device_name: String,
    /// 设备类型
    pub device_type: String,
    /// 设备状态
    pub status: String,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
}

/// 用户基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBase {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub name: String,
    /// 用户状态
    pub status: crate::models::Status,
    /// 部门ID列表
    pub department_ids: Vec<String>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
}

/// 权限基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionBase {
    /// 权限ID
    pub permission_id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: Option<String>,
    /// 权限状态
    pub status: crate::models::Status,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
}

/// 操作结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResponse {
    /// 是否成功
    pub success: bool,
    /// 操作ID
    pub operation_id: Option<String>,
    /// 响应消息
    pub message: Option<String>,
}

/// 批量操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationRequest<T> {
    /// 操作项列表
    pub items: Vec<T>,
    /// 是否跳过错误
    pub skip_error: Option<bool>,
}

/// 批量操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResponse<T> {
    /// 成功项目列表
    pub success_items: Vec<T>,
    /// 失败项目列表
    pub failed_items: Vec<BatchOperationError<T>>,
}

/// 批量操作错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationError<T> {
    /// 错误项目
    pub item: T,
    /// 错误码
    pub error_code: i32,
    /// 错误消息
    pub error_message: String,
}

/// 查询条件枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryCondition {
    /// 等于
    Eq(String, serde_json::Value),
    /// 不等于
    Ne(String, serde_json::Value),
    /// 大于
    Gt(String, serde_json::Value),
    /// 大于等于
    Gte(String, serde_json::Value),
    /// 小于
    Lt(String, serde_json::Value),
    /// 小于等于
    Lte(String, serde_json::Value),
    /// 包含
    Contains(String, serde_json::Value),
    /// 在范围内
    In(String, Vec<serde_json::Value>),
    /// 不在范围内
    NotIn(String, Vec<serde_json::Value>),
}

/// 排序条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortCondition {
    /// 排序字段
    pub field: String,
    /// 排序方向
    pub direction: SortDirection,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 时间范围查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// 开始时间
    pub start_time: crate::models::Timestamp,
    /// 结束时间
    pub end_time: crate::models::Timestamp,
}

/// 地理位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 地址描述
    pub address: Option<String>,
}

/// 文件上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// 文件ID
    pub file_id: String,
    /// 文件URL
    pub file_url: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub upload_time: crate::models::Timestamp,
}
