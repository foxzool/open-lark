//! Bitable App Table Form Field API 数据模型
//!
//! 提供多维表格表单字段管理相关的数据结构，支持表单字段的查询、
//! 更新等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 列出表单字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFormFieldRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListFormFieldRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.form_id.trim().is_empty() {
            return Err("表单ID不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出表单字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListFormFieldResponse {
    /// 表单字段信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FormFieldInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListFormFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单字段请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
    /// 字段更新列表
    pub fields: Vec<FormFieldUpdate>,
}

impl PatchFormFieldRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.form_id.trim().is_empty() {
            return Err("表单ID不能为空".to_string());
        }
        if self.fields.is_empty() {
            return Err("字段更新列表不能为空".to_string());
        }
        if self.fields.len() > 200 {
            return Err("字段更新列表不能超过200个".to_string());
        }
        for (index, field) in self.fields.iter().enumerate() {
            if let Err(e) = field.validate() {
                return Err(format!("字段{}验证失败: {}", index + 1, e));
            }
        }
        Ok(())
    }
}

/// 更新表单字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchFormFieldResponse {
    /// 表单字段信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FormFieldInfo>>,
}

impl ApiResponseTrait for PatchFormFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 表单字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldInfo {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 字段配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 字段顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Value>,
    /// 字段验证规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<Value>,
    /// 字段状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 表单字段更新
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldUpdate {
    /// 字段ID
    pub field_id: String,
    /// 字段名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 是否必填（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 字段描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 字段配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 默认值（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Value>,
    /// 字段验证规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<Value>,
    /// 字段状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl FormFieldUpdate {
    /// 验证字段更新参数
    pub fn validate(&self) -> Result<(), String> {
        if self.field_id.trim().is_empty() {
            return Err("字段ID不能为空".to_string());
        }
        if let Some(ref name) = self.field_name {
            if name.trim().is_empty() {
                return Err("字段名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("字段名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("字段描述不能超过1000个字符".to_string());
            }
        }
        Ok(())
    }
}
