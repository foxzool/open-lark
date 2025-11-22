//! Bitable App Table Record API 数据模型
//!
//! 提供多维表格数据记录管理相关的数据结构，支持记录的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 新增记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录字段值
    pub fields: Value,
}

impl CreateRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if !self.fields.is_object() {
            return Err("字段值必须是对象格式".to_string());
        }
        Ok(())
    }
}

/// 新增记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateRecordResponse {
    /// 记录信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordInfo>,
}

impl ApiResponseTrait for CreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量新增记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录列表
    pub records: Vec<CreateRecordRequest>,
}

impl BatchCreateRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.records.is_empty() {
            return Err("记录列表不能为空".to_string());
        }
        if self.records.len() > 500 {
            return Err("单次批量创建记录数量不能超过500个".to_string());
        }
        for (i, record) in self.records.iter().enumerate() {
            record
                .validate()
                .map_err(|e| format!("第{}个记录验证失败: {}", i + 1, e))?;
        }
        Ok(())
    }
}

/// 批量新增记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchCreateRecordResponse {
    /// 记录信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInfo>>,
}

impl ApiResponseTrait for BatchCreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录ID
    pub record_id: String,
    /// 记录字段值
    pub fields: Value,
}

impl UpdateRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.record_id.trim().is_empty() {
            return Err("记录ID不能为空".to_string());
        }
        if !self.fields.is_object() {
            return Err("字段值必须是对象格式".to_string());
        }
        Ok(())
    }
}

/// 更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateRecordResponse {
    /// 记录信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordInfo>,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录列表
    pub records: Vec<UpdateRecordRequest>,
}

impl BatchUpdateRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.records.is_empty() {
            return Err("记录列表不能为空".to_string());
        }
        if self.records.len() > 500 {
            return Err("单次批量更新记录数量不能超过500个".to_string());
        }
        for (i, record) in self.records.iter().enumerate() {
            record
                .validate()
                .map_err(|e| format!("第{}个记录验证失败: {}", i + 1, e))?;
        }
        Ok(())
    }
}

/// 批量更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateRecordResponse {
    /// 记录信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInfo>>,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录ID
    pub record_id: String,
}

impl DeleteRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.record_id.trim().is_empty() {
            return Err("记录ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteRecordResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录ID列表
    pub record_ids: Vec<String>,
}

impl BatchDeleteRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.record_ids.is_empty() {
            return Err("记录ID列表不能为空".to_string());
        }
        if self.record_ids.len() > 500 {
            return Err("单次批量删除记录数量不能超过500个".to_string());
        }
        Ok(())
    }
}

/// 批量删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchDeleteRecordResponse {
    /// 删除结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DeleteResult>>,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录ID
    pub record_id: String,
    /// 字段类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否需要字段的名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_field_name: Option<bool>,
}

impl GetRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.record_id.trim().is_empty() {
            return Err("记录ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 查询记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetRecordResponse {
    /// 记录信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordInfo>,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRecordsRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 字段类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否需要字段的名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_field_name: Option<bool>,
    /// 排序字段（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortInfo>>,
    /// 过滤条件（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterInfo>,
}

impl ListRecordsRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListRecordsResponse {
    /// 记录信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListRecordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRecordRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 记录ID列表
    pub record_ids: Vec<String>,
    /// 字段类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否需要字段的名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_field_name: Option<bool>,
}

impl BatchGetRecordRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.record_ids.is_empty() {
            return Err("记录ID列表不能为空".to_string());
        }
        if self.record_ids.len() > 500 {
            return Err("单次批量获取记录数量不能超过500个".to_string());
        }
        Ok(())
    }
}

/// 批量获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchGetRecordResponse {
    /// 记录信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInfo>>,
}

impl ApiResponseTrait for BatchGetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索记录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRecordsRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 搜索字符串
    pub search_string: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 字段类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否需要字段的名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_field_name: Option<bool>,
    /// 排序字段（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortInfo>>,
}

impl SearchRecordsRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.search_string.trim().is_empty() {
            return Err("搜索字符串不能为空".to_string());
        }
        if self.search_string.len() > 1000 {
            return Err("搜索字符串不能超过1000个字符".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 搜索记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchRecordsResponse {
    /// 记录信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SearchRecordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecordInfo {
    /// 记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 更新者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater: Option<UserInfo>,
}

/// 排序信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortInfo {
    /// 字段名
    pub field_name: String,
    /// 排序方向
    pub desc: bool,
}

/// 过滤条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterInfo {
    /// 条件列表
    pub conditions: Vec<FilterCondition>,
    /// 条件关系（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
}

/// 过滤条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    /// 字段名
    pub field_name: String,
    /// 操作符
    pub operator: String,
    /// 值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// 删除结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResult {
    /// 记录ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 用户信息（重用自其他模块）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}
