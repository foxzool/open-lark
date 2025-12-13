/// CCM Doc V1 响应类型定义
use serde::{Deserialize, Serialize};

/// 创建的文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedDocument {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 父文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
}

/// 文档元信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetaData {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<User>,
    /// 更新者信息
    pub updater: Option<User>,
    /// 父文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
}

/// 电子表格元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetMetaData {
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 工作表数量
    #[serde(rename = "sheet_count")]
    pub sheet_count: i32,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    #[serde(rename = "sheet_type")]
    pub sheet_type: String,
    /// 行数
    #[serde(rename = "row_count")]
    pub row_count: i32,
    /// 列数
    #[serde(rename = "column_count")]
    pub column_count: i32,
}

/// 纯文本内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawContentData {
    /// 纯文本内容
    pub content: String,
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 内容长度
    pub length: i32,
}

/// 富文本内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContentData {
    /// 文档内容
    pub content: serde_json::Value,
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 内容元素数量
    #[serde(rename = "element_count")]
    pub element_count: i32,
    /// 版本号
    pub revision: i32,
}

/// 批量更新数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateData {
    /// 是否成功
    pub success: bool,
    /// 更新后的文档token
    #[serde(rename = "doc_token")]
    pub doc_token: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 更新后的版本号
    pub revision: Option<i32>,
}

/// 用户信息 (重导出模型中的定义)
pub use super::models::User;