//! 云文档模型定义

use serde::{Deserialize, Serialize};

/// 文件类型
#[derive(Debug, Deserialize, Serialize)]
pub enum FileType {
    Document,
    Spreadsheet,
    Presentation,
    Image,
    Video,
    Other,
}

/// 云文档响应
#[derive(Debug, Deserialize, Serialize)]
pub struct CcmResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 通用分页响应
#[derive(Debug, Deserialize, Serialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub page_token: Option<String>,
    pub has_more: bool,
}
