//! Bitable App Form API 数据模型
//!
//! 提供多维表格表单管理相关的数据结构，支持表单的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取表单请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFormRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
}

impl GetFormRequest {
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
        Ok(())
    }
}

/// 获取表单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetFormResponse {
    /// 表单信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<FormInfo>,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出表单问题请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFormQuestionsRequest {
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

impl ListFormQuestionsRequest {
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

/// 列出表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListFormQuestionsResponse {
    /// 表单问题列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<FormQuestion>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListFormQuestionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单问题请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormQuestionRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
    /// 问题ID
    pub question_id: String,
    /// 问题标题
    pub title: String,
    /// 问题描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 问题类型
    pub question_type: String,
    /// 是否必填
    pub required: bool,
    /// 选项配置（可选，用于选择题）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<QuestionOption>>,
    /// 验证规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationRule>,
}

impl PatchFormQuestionRequest {
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
        if self.question_id.trim().is_empty() {
            return Err("问题ID不能为空".to_string());
        }
        if self.title.trim().is_empty() {
            return Err("问题标题不能为空".to_string());
        }
        if self.title.len() > 200 {
            return Err("问题标题不能超过200个字符".to_string());
        }
        if self.question_type.trim().is_empty() {
            return Err("问题类型不能为空".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("问题描述不能超过1000个字符".to_string());
            }
        }
        if let Some(ref options) = self.options {
            if options.len() > 100 {
                return Err("选项数量不能超过100个".to_string());
            }
        }
        Ok(())
    }
}

/// 更新表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchFormQuestionResponse {
    /// 表单问题信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<FormQuestion>,
}

impl ApiResponseTrait for PatchFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormMetaRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
    /// 表单标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 表单描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 表单配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<FormSettings>,
}

impl PatchFormMetaRequest {
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
        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("表单标题不能为空".to_string());
            }
            if title.len() > 100 {
                return Err("表单标题不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("表单描述不能超过1000个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新表单元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchFormMetaResponse {
    /// 表单信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<FormInfo>,
}

impl ApiResponseTrait for PatchFormMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 表单信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormInfo {
    /// 表单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    /// 表单标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 表单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 应用token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_token: Option<String>,
    /// 表单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FormStatus>,
    /// 表单设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<FormSettings>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
}

/// 表单状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FormStatus {
    /// 草稿
    #[serde(rename = "draft")]
    Draft,
    /// 已发布
    #[serde(rename = "published")]
    Published,
    /// 已暂停
    #[serde(rename = "paused")]
    Paused,
}

/// 表单设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormSettings {
    /// 是否允许重复提交
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_duplicate_submit: Option<bool>,
    /// 提交后是否显示结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_result_after_submit: Option<bool>,
    /// 提交成功消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_message: Option<String>,
    /// 表单样式配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<FormStyle>,
}

/// 表单样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormStyle {
    /// 主题颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<String>,
    /// 背景图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    /// 字体大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
}

/// 表单问题
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormQuestion {
    /// 问题ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    /// 问题标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 问题类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 选项列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<QuestionOption>>,
    /// 验证规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationRule>,
    /// 问题序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 问题选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuestionOption {
    /// 选项ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// 选项文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 选项值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 验证规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ValidationRule {
    /// 最小长度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    /// 最大长度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    /// 正则表达式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
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

/// 问题类型常量
pub mod question_types {
    /// 单行文本
    pub const TEXT: &str = "text";
    /// 多行文本
    pub const PARAGRAPH: &str = "paragraph";
    /// 数字
    pub const NUMBER: &str = "number";
    /// 邮箱
    pub const EMAIL: &str = "email";
    /// 电话
    pub const PHONE: &str = "phone";
    /// 日期
    pub const DATE: &str = "date";
    /// 时间
    pub const TIME: &str = "time";
    /// 单选题
    pub const RADIO: &str = "radio";
    /// 多选题
    pub const CHECKBOX: &str = "checkbox";
    /// 下拉选择
    pub const SELECT: &str = "select";
    /// 评分
    pub const RATING: &str = "rating";
    /// 文件上传
    pub const FILE: &str = "file";
}
