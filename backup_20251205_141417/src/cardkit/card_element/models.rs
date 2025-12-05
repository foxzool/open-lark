//! Card Element API 数据模型
//!
//! 提供卡片组件管理相关的数据结构，支持组件的创建、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建卡片组件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件数据
    pub element: Value,
    /// 组件类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    /// 组件标签（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl CreateCardElementRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        // 验证组件数据不为空
        if self.element.is_null() {
            return Err("组件数据不能为空".to_string());
        }

        // 验证组件数据是否为有效对象
        if !self.element.is_object() {
            return Err("组件数据必须是有效的JSON对象".to_string());
        }

        Ok(())
    }
}

/// 创建卡片组件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateCardElementResponse {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl ApiResponseTrait for CreateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新卡片组件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 组件数据
    pub element: Value,
    /// 更新掩码（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Vec<String>>,
}

impl UpdateCardElementRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        if self.element_id.trim().is_empty() {
            return Err("组件ID不能为空".to_string());
        }

        // 验证组件数据不为空
        if self.element.is_null() {
            return Err("组件数据不能为空".to_string());
        }

        // 验证组件数据是否为有效对象
        if !self.element.is_object() {
            return Err("组件数据必须是有效的JSON对象".to_string());
        }

        Ok(())
    }
}

/// 更新卡片组件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateCardElementResponse {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 局部更新卡片组件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 组件属性
    pub properties: Value,
}

impl PatchCardElementRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        if self.element_id.trim().is_empty() {
            return Err("组件ID不能为空".to_string());
        }

        // 验证属性数据不为空
        if self.properties.is_null() {
            return Err("组件属性不能为空".to_string());
        }

        // 验证属性数据是否为有效对象
        if !self.properties.is_object() {
            return Err("组件属性必须是有效的JSON对象".to_string());
        }

        Ok(())
    }
}

/// 局部更新卡片组件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchCardElementResponse {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl ApiResponseTrait for PatchCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新组件内容请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCardElementContentRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 内容数据
    pub content: Value,
    /// 是否追加内容（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    /// 是否启用流式更新（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl UpdateCardElementContentRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        if self.element_id.trim().is_empty() {
            return Err("组件ID不能为空".to_string());
        }

        // 验证内容数据
        if self.content.is_null() {
            return Err("内容数据不能为空".to_string());
        }

        // 对于文本组件，内容应该是字符串
        if self.content.is_string() {
            let content_str = self.content.as_str().unwrap_or("");
            if content_str.len() > 10000 {
                return Err("内容长度不能超过10000个字符".to_string());
            }
        }

        Ok(())
    }
}

/// 更新组件内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateCardElementContentResponse {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for UpdateCardElementContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除卡片组件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
}

impl DeleteCardElementRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("卡片ID不能为空".to_string());
        }

        if self.element_id.trim().is_empty() {
            return Err("组件ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 删除卡片组件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteCardElementResponse {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 卡片组件信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CardElementInfo {
    /// 组件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// 组件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    /// 组件标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 组件数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Value>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 组件位置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ElementPosition {
    /// 行索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<i32>,
    /// 列索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    /// 宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 组件样式配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ElementStyle {
    /// 背景颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// 边框颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    /// 边框宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<i32>,
    /// 圆角半径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<i32>,
    /// 内边距
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<i32>,
    /// 外边距
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<i32>,
}

/// 组件类型常量
pub mod element_types {
    /// 文本组件
    pub const TEXT: &str = "text";
    /// 图片组件
    pub const IMAGE: &str = "image";
    /// 按钮组件
    pub const BUTTON: &str = "button";
    /// 输入框组件
    pub const INPUT: &str = "input";
    /// 选择器组件
    pub const SELECT: &str = "select";
    /// 复选框组件
    pub const CHECKBOX: &str = "checkbox";
    /// 单选框组件
    pub const RADIO: &str = "radio";
    /// 分割线组件
    pub const DIVIDER: &str = "divider";
    /// 标题组件
    pub const TITLE: &str = "title";
    /// 富文本组件
    pub const RICH_TEXT: &str = "rich_text";
    /// 表格组件
    pub const TABLE: &str = "table";
    /// 图表组件
    pub const CHART: &str = "chart";
    /// 容器组件
    pub const CONTAINER: &str = "container";
}

/// 组件标签常量
pub mod element_tags {
    /// 必填
    pub const REQUIRED: &str = "required";
    /// 只读
    pub const READONLY: &str = "readonly";
    /// 隐藏
    pub const HIDDEN: &str = "hidden";
    /// 禁用
    pub const DISABLED: &str = "disabled";
    /// 主按钮
    pub const PRIMARY: &str = "primary";
    /// 次要按钮
    pub const SECONDARY: &str = "secondary";
    /// 警告按钮
    pub const WARNING: &str = "warning";
    /// 危险按钮
    pub const DANGER: &str = "danger";
}
