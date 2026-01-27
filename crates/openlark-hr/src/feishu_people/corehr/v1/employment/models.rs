//! CoreHR 雇佣信息相关模型
//!
//! 包含创建、删除、更新雇佣信息等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// ============================================================================
// 雇佣信息基础数据结构
// ============================================================================

/// 雇佣信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employment {
    /// 雇佣 ID
    pub employment_id: String,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 离职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 雇佣状态
    /// - 1: 试用期
    /// - 2: 在职
    /// - 3: 离职
    /// - 4: 退休
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 雇佣类型
    /// - 1: 正式员工
    /// - 2: 临时员工
    /// - 3: 兼职员工
    /// - 4: 实习生
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomField {
    /// 字段 API 名称
    pub field_api_name: String,
    /// 字段值
    pub field_value: serde_json::Value,
}

// ============================================================================
// 创建雇佣信息相关模型
// ============================================================================

/// 创建雇佣信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 员工 ID（必填）
    pub employee_id: String,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 离职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 雇佣状态
    /// - 1: 试用期
    /// - 2: 在职
    /// - 3: 离职
    /// - 4: 退休
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 雇佣类型
    /// - 1: 正式员工
    /// - 2: 临时员工
    /// - 3: 兼职员工
    /// - 4: 实习生
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建雇佣信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 雇佣 ID
    pub employment_id: String,
}

// ============================================================================
// 删除雇佣信息相关模型
// ============================================================================

/// 删除雇佣信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 雇佣 ID（必填）
    pub employment_id: String,
}

/// 删除雇佣信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// ============================================================================
// 更新雇佣信息相关模型
// ============================================================================

/// 更新雇佣信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 雇佣 ID（必填）
    pub employment_id: String,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 离职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 雇佣状态
    /// - 1: 试用期
    /// - 2: 在职
    /// - 3: 离职
    /// - 4: 退休
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 雇佣类型
    /// - 1: 正式员工
    /// - 2: 临时员工
    /// - 3: 兼职员工
    /// - 4: 实习生
    /// - 5: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 更新雇佣信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}
