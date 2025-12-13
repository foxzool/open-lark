/// CCM Doc V2 数据模型
use serde::{Deserialize, Serialize};

/// 创建旧版文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentParams {
    /// 文档标题，长度限制：1-100字符
    pub title: String,
    /// 父文件夹token，不填则存在"我的空间"
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 文档类型，可选值：doc、sheet、bitable、mindnote、file
    #[serde(rename = "parent_type")]
    pub parent_type: Option<String>,
}

/// 创建旧版文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
    /// 文档信息
    pub data: Option<DocumentData>,
}

/// 文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
}

/// 文档元信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetaResponse {
    /// 文档元信息
    pub data: Option<DocumentMeta>,
}

/// 文档元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
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
    pub creator: Option<UserInfo>,
    /// 更新者信息
    pub updater: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名称
    pub name: String,
}

/// 电子表格元信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetMetaResponse {
    /// 电子表格元信息
    pub data: Option<SheetMeta>,
}

/// 电子表格元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetMeta {
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
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
}

/// 文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawContentResponse {
    /// 纯文本内容
    pub data: Option<RawContentData>,
}

/// 纯文本内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawContentData {
    /// 纯文本内容
    pub content: String,
}

/// 文档富文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContentResponse {
    /// 富文本内容
    pub data: Option<DocumentContent>,
}

/// 文档富文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    /// 文档内容
    pub content: serde_json::Value,
}

/// 批量更新文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateParams {
    /// 更新操作列表
    pub operations: Vec<DocumentOperation>,
}

/// 文档操作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation_type")]
pub enum DocumentOperation {
    /// 插入文本
    #[serde(rename = "insert")]
    Insert {
        /// 插入位置
        #[serde(rename = "location")]
        location: Location,
        /// 插入内容
        text: String,
    },
    /// 删除内容
    #[serde(rename = "delete")]
    Delete {
        /// 删除范围
        #[serde(rename = "range")]
        range: Range,
    },
    /// 替换内容
    #[serde(rename = "replace")]
    Replace {
        /// 替换范围
        #[serde(rename = "range")]
        range: Range,
        /// 新内容
        text: String,
    },
    /// 更新标题
    #[serde(rename = "update_title")]
    UpdateTitle {
        /// 新标题
        title: String,
    },
}

/// 位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 段落索引
    #[serde(rename = "segment_id")]
    pub segment_id: i32,
    /// 字符偏移量
    #[serde(rename = "offset")]
    pub offset: i32,
}

/// 范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// 起始位置
    #[serde(rename = "start")]
    pub start: Location,
    /// 结束位置
    #[serde(rename = "end")]
    pub end: Location,
}

/// 批量更新文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateResponse {
    /// 更新结果
    pub data: Option<BatchUpdateData>,
}

/// 批量更新数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateData {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}
