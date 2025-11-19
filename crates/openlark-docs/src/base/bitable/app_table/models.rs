//! Bitable App Table API 数据模型
//!
//! 提供多维表格数据表管理相关的数据结构，支持数据表的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 新增一个数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTableRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表名称
    pub name: String,
    /// 数据表描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用高级模式（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    /// 数据表字段定义（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldInfo>>,
}

impl CreateTableRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("数据表名称不能为空".to_string());
        }
        if self.name.len() > 100 {
            return Err("数据表名称不能超过100个字符".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("数据表描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 新增一个数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateTableResponse {
    /// 数据表信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableInfo>,
}

impl ApiResponseTrait for CreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增多个数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateTableRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表列表
    pub tables: Vec<CreateTableRequest>,
}

impl BatchCreateTableRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.tables.is_empty() {
            return Err("数据表列表不能为空".to_string());
        }
        if self.tables.len() > 50 {
            return Err("单次批量创建数据表数量不能超过50个".to_string());
        }
        for (i, table) in self.tables.iter().enumerate() {
            table
                .validate()
                .map_err(|e| format!("第{}个数据表验证失败: {}", i + 1, e))?;
        }
        Ok(())
    }
}

/// 新增多个数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchCreateTableResponse {
    /// 数据表信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableInfo>>,
}

impl ApiResponseTrait for BatchCreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateTableRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 数据表名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据表描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用高级模式（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
}

impl UpdateTableRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("数据表名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("数据表名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("数据表描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateTableResponse {
    /// 数据表信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableInfo>,
}

impl ApiResponseTrait for UpdateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTablesRequest {
    /// 应用token
    pub app_token: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListTablesRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListTablesResponse {
    /// 数据表信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteTableRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
}

impl DeleteTableRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteTableResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除多个数据表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteTableRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID列表
    pub table_ids: Vec<String>,
}

impl BatchDeleteTableRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_ids.is_empty() {
            return Err("数据表ID列表不能为空".to_string());
        }
        if self.table_ids.len() > 50 {
            return Err("单次批量删除数据表数量不能超过50个".to_string());
        }
        Ok(())
    }
}

/// 删除多个数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchDeleteTableResponse {
    /// 删除结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DeleteResult>>,
}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TableInfo {
    /// 数据表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 数据表名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据表描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用高级模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    /// 数据表时间区域
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 数据表字段数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_count: Option<i32>,
    /// 数据表记录数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FieldInfo {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// 是否唯一
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
}

/// 删除结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResult {
    /// 数据表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 用户信息（重用自app模块）
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
