//! 表单问题相关数据模型
//!
//! 定义了表单问题的数据结构和相关类型

use serde_json::Value;
use serde::{Deserialize, Serialize};

/// 表单问题类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormQuestionType {
    /// 单行文本
    #[serde(rename = "text")]
    Text,
    /// 多行文本
    #[serde(rename = "textarea")]
    Textarea,
    /// 数字
    #[serde(rename = "number")]
    Number,
    /// 日期
    #[serde(rename = "date")]
    Date,
    /// 单选
    #[serde(rename = "single_select")]
    SingleSelect,
    /// 多选
    #[serde(rename = "multi_select")]
    MultiSelect,
    /// 附件
    #[serde(rename = "attachment")]
    Attachment,
    /// 人员
    #[serde(rename = "member")]
    Member,
    /// 复选框
    #[serde(rename = "checkbox")]
    Checkbox,
    /// 评分
    #[serde(rename = "rating")]
    Rating,
    /// URL
    #[serde(rename = "url")]
    Url,
    /// 邮箱
    #[serde(rename = "email")]
    Email,
    /// 电话
    #[serde(rename = "phone")]
    Phone,
}

/// 表单问题选项（用于单选、多选类型）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormQuestionOption {
    /// 选项ID
    pub id: String,
    /// 选项名称
    pub name: String,
    /// 选项值
    pub value: String,
    /// 是否禁用
    #[serde(default)]
    pub disabled: bool,
}

/// 表单问题验证规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormQuestionValidation {
    /// 是否必填
    pub required: bool,
    /// 最小长度（文本类型）
    pub min_length: Option<i32>,
    /// 最大长度（文本类型）
    pub max_length: Option<i32>,
    /// 最小值（数字类型）
    pub min_value: Option<f64>,
    /// 最大值（数字类型）
    pub max_value: Option<f64>,
    /// 正则表达式（文本类型）
    pub pattern: Option<String>,
    /// 错误提示信息
    pub error_message: Option<String>,
}

/// 表单问题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormQuestionConfig {
    /// 问题类型
    pub question_type: FormQuestionType,
    /// 问题选项（用于单选、多选类型）
    pub options: Option<Vec<FormQuestionOption>>,
    /// 验证规则
    pub validation: Option<FormQuestionValidation>,
    /// 默认值
    pub default_value: Option<serde_json::Value>,
    /// 占位符文本
    pub placeholder: Option<String>,
    /// 帮助文本
    pub help_text: Option<String>,
}

/// 表单问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormQuestion {
    /// 问题ID
    pub question_id: String,
    /// 问题标题
    pub title: String,
    /// 问题描述
    pub description: Option<String>,
    /// 问题类型
    pub question_type: FormQuestionType,
    /// 是否必填
    pub required: bool,
    /// 是否可见
    pub visible: bool,
    /// 问题配置
    pub config: FormQuestionConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
    /// 排序序号
    pub sort_order: i32,
}