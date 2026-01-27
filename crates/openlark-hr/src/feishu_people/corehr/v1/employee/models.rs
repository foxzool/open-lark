//! CoreHR 员工相关模型
//!
//! 包含创建、删除、查询、搜索员工等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// ============================================================================
// 员工基础数据结构
// ============================================================================

/// 员工信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    /// 员工 ID
    pub employee_id: String,
    /// 员工姓名
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工状态
    /// - 0: 未入职
    /// - 1: 在职
    /// - 2: 离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 离职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

/// 员工花名册信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeRoster {
    /// 员工 ID
    pub employee_id: String,
    /// 员工姓名
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_name: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

// ============================================================================
// 创建员工相关模型
// ============================================================================

/// 创建员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 员工姓名（必填）
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

/// 创建员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 员工 ID
    pub employee_id: String,
}

// ============================================================================
// 删除员工相关模型
// ============================================================================

/// 删除员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 员工 ID（必填）
    pub employee_id: String,
}

/// 删除员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// ============================================================================
// 批量查询员工花名册相关模型
// ============================================================================

/// 批量查询员工花名册请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询员工花名册响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 员工花名册列表
    pub items: Vec<EmployeeRoster>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// ============================================================================
// 搜索员工相关模型
// ============================================================================

/// 搜索员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequestBody {
    /// 搜索关键词（必填）
    pub query: String,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 员工列表
    pub items: Vec<Employee>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// ============================================================================
// 批量获取员工相关模型
// ============================================================================

/// 批量获取员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRequestBody {
    /// 员工 ID 列表（必填，最多 100 个）
    pub employee_ids: Vec<String>,
}

/// 批量获取员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 员工列表
    pub items: Vec<Employee>,
}
