//! Docx模块公共类型定义
//!
//! 统一管理docx相关API中使用的公共数据结构，避免重复定义。

use serde::{Deserialize, Serialize};

/// 块内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContent {
    /// 文本内容
    pub text: Option<String>,
    /// 富文本内容
    pub rich_text: Option<RichText>,
    /// 其他类型内容
    #[serde(flatten)]
    pub other: Option<serde_json::Value>,
}

/// 富文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichText {
    /// 段落列表
    pub paragraphs: Vec<Paragraph>,
}

/// 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    /// 文本元素列表
    pub elements: Vec<TextElement>,
}

/// 文本元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    /// 文本
    pub text_run: Option<TextRun>,
}

/// 文本运行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    /// 内容
    pub content: String,
    /// 样式
    pub style: Option<TextStyle>,
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// 是否加粗
    pub bold: Option<bool>,
    /// 是否斜体
    pub italic: Option<bool>,
    /// 是否删除线
    pub strikethrough: Option<bool>,
    /// 字体大小
    pub font_size: Option<u32>,
    /// 字体颜色
    pub font_color: Option<String>,
}

/// 块项目基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockItem {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: String,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 子块数量
    pub children_count: Option<u32>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 分页数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

/// 块更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockUpdate {
    /// 块ID
    pub block_id: String,
    /// 更新的内容
    pub content: Option<BlockContent>,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 成功数量
    pub success_count: Option<u32>,
    /// 失败数量
    pub failed_count: Option<u32>,
    /// 失败的项目
    pub failed_items: Option<Vec<FailedItem>>,
}

/// 失败项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedItem {
    /// 项目ID
    pub item_id: String,
    /// 错误信息
    pub error_message: String,
    /// 错误代码
    pub error_code: Option<i32>,
}
