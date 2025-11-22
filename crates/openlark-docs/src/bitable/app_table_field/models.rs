//! Bitable App Table Field API 数据模型
//!
//! 提供多维表格字段管理相关的数据结构，支持字段的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 新增字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFieldRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 字段属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 字段描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必填（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

impl CreateFieldRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.field_name.trim().is_empty() {
            return Err("字段名称不能为空".to_string());
        }
        if self.field_name.len() > 100 {
            return Err("字段名称不能超过100个字符".to_string());
        }
        if self.field_type.trim().is_empty() {
            return Err("字段类型不能为空".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("字段描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 新增字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateFieldResponse {
    /// 字段信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldInfo>,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateFieldRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 字段ID
    pub field_id: String,
    /// 字段名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段属性（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 字段描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必填（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

impl UpdateFieldRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.field_id.trim().is_empty() {
            return Err("字段ID不能为空".to_string());
        }
        if let Some(ref field_name) = self.field_name {
            if field_name.trim().is_empty() {
                return Err("字段名称不能为空".to_string());
            }
            if field_name.len() > 100 {
                return Err("字段名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("字段描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateFieldResponse {
    /// 字段信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldInfo>,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteFieldRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 字段ID
    pub field_id: String,
}

impl DeleteFieldRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.field_id.trim().is_empty() {
            return Err("字段ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteFieldResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFieldsRequest {
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
}

impl ListFieldsRequest {
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

/// 列出字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListFieldsResponse {
    /// 字段信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FieldInfo {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 字段属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// 是否唯一
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    /// 字段序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 字段类型常量
pub mod field_types {
    /// 文本类型
    pub const TEXT: &str = "text";
    /// 数字类型
    pub const NUMBER: &str = "number";
    /// 单选类型
    pub const SELECT: &str = "select";
    /// 多选类型
    pub const MULTI_SELECT: &str = "multi_select";
    /// 日期类型
    pub const DATE: &str = "date";
    /// 日期时间类型
    pub const DATETIME: &str = "datetime";
    /// 用户类型
    pub const USER: &str = "user";
    /// 多用户类型
    pub const MULTI_USER: &str = "multi_user";
    /// 附件类型
    pub const ATTACHMENT: &str = "attachment";
    /// 复选框类型
    pub const CHECKBOX: &str = "checkbox";
    /// URL类型
    pub const URL: &str = "url";
    /// 电话类型
    pub const PHONE: &str = "phone";
    /// 邮箱类型
    pub const EMAIL: &str = "email";
    /// 地点类型
    pub const LOCATION: &str = "location";
    /// 公式类型
    pub const FORMULA: &str = "formula";
    /// 查找引用类型
    pub const LOOKUP: &str = "lookup";
    /// 级联选择类型
    pub const CASCADER: &str = "cascader";
    /// 进度条类型
    pub const PROGRESS: &str = "progress";
    /// 评分类型
    pub const RATING: &str = "rating";
    /// 货币类型
    pub const CURRENCY: &str = "currency";
    /// 自动编号类型
    pub const AUTO_NUMBER: &str = "auto_number";
    /// 创建时间类型
    pub const CREATED_TIME: &str = "created_time";
    /// 最后更新时间类型
    pub const LAST_MODIFIED_TIME: &str = "last_modified_time";
    /// 创建人类型
    pub const CREATED_BY: &str = "created_by";
    /// 最后更新人类型
    pub const LAST_MODIFIED_BY: &str = "last_modified_by";
}
