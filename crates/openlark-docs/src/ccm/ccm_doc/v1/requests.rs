/// CCM Doc V1 请求类型定义
use serde::{Deserialize, Serialize};
use openlark_core::api::ApiRequest;

// Re-export types from models to maintain compatibility
pub use super::models::{BatchUpdateParams, DocumentContent, DocumentMeta, SheetInfo, User};

/// 创建旧版文档请求
#[derive(Debug, Clone)]
pub struct CreateDocumentRequest {
    /// 文档标题，长度限制：1-100字符
    pub title: String,
    /// 父文件夹token，不填则存在"我的空间"
    pub folder_token: Option<String>,
    /// 文档类型，可选值：doc、sheet、bitable、mindnote、file
    pub parent_type: Option<String>,
}

impl CreateDocumentRequest {
    /// 创建新的创建文档请求
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            folder_token: None,
            parent_type: None,
        }
    }

    /// 设置父文件夹token
    pub fn folder_token(mut self, folder_token: &str) -> Self {
        self.folder_token = Some(folder_token.to_string());
        self
    }

    /// 设置文档类型
    pub fn parent_type(mut self, parent_type: &str) -> Self {
        self.parent_type = Some(parent_type.to_string());
        self
    }
}

/// 获取文档元信息请求
#[derive(Debug, Clone)]
pub struct DocumentMetaRequest {
    /// 文档token
    pub document_token: String,
}

impl DocumentMetaRequest {
    /// 创建新的获取文档元信息请求
    pub fn new(document_token: &str) -> Self {
        Self {
            document_token: document_token.to_string(),
        }
    }
}

/// 获取电子表格元数据请求
#[derive(Debug, Clone)]
pub struct SheetMetaRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
}

impl SheetMetaRequest {
    /// 创建新的获取电子表格元数据请求
    pub fn new(spreadsheet_token: &str) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.to_string(),
        }
    }
}

/// 获取纯文本内容请求
#[derive(Debug, Clone)]
pub struct RawContentRequest {
    /// 文档token
    pub document_token: String,
}

impl RawContentRequest {
    /// 创建新的获取纯文本内容请求
    pub fn new(document_token: &str) -> Self {
        Self {
            document_token: document_token.to_string(),
        }
    }
}

/// 获取富文本内容请求
#[derive(Debug, Clone)]
pub struct DocumentContentRequest {
    /// 文档token
    pub document_token: String,
}

impl DocumentContentRequest {
    /// 创建新的获取富文本内容请求
    pub fn new(document_token: &str) -> Self {
        Self {
            document_token: document_token.to_string(),
        }
    }
}

/// 批量更新文档请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRequest {
    /// 文档token
    pub document_token: String,
    /// 更新操作列表
    pub operations: Vec<DocumentOperation>,
}

impl BatchUpdateRequest {
    /// 创建新的批量更新文档请求
    pub fn new(document_token: &str) -> Self {
        Self {
            document_token: document_token.to_string(),
            operations: Vec::new(),
        }
    }

    /// 添加更新操作
    pub fn add_operation(mut self, operation: DocumentOperation) -> Self {
        self.operations.push(operation);
        self
    }
}

/// 文档操作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation_type")]
pub enum DocumentOperation {
    /// 插入文本
    #[serde(rename = "insert")]
    Insert {
        /// 插入位置
        location: Location,
        /// 插入内容
        text: String,
    },
    /// 删除内容
    #[serde(rename = "delete")]
    Delete {
        /// 删除范围
        range: Range,
    },
    /// 替换内容
    #[serde(rename = "replace")]
    Replace {
        /// 替换范围
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
    pub segment_id: i32,
    /// 字符偏移量
    pub offset: i32,
}

impl Location {
    /// 创建新的位置
    pub fn new(segment_id: i32, offset: i32) -> Self {
        Self { segment_id, offset }
    }
}

/// 范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// 起始位置
    pub start: Location,
    /// 结束位置
    pub end: Location,
}

impl Range {
    /// 创建新的范围
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
}