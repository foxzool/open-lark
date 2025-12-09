//! Docx API 数据模型

use serde::{Deserialize, Serialize};

/// 群公告基础信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AnnouncementInfo {
    /// 群公告ID
    pub announcement_id: String,
    /// 群ID
    pub chat_id: String,
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 群公告块信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AnnouncementBlockInfo {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: serde_json::Value,
    /// 父块ID
    pub parent_block_id: Option<String>,
    /// 子块ID列表
    pub child_block_ids: Vec<String>,
}

/// 文档基础信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DocumentInfo {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub doc_type: String,
    /// 创建者ID
    pub creator_id: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 文档块信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DocumentBlockInfo {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: serde_json::Value,
    /// 父块ID
    pub parent_block_id: Option<String>,
    /// 子块ID列表
    pub child_block_ids: Vec<String>,
}

/// 删除操作结果
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DeleteResult {
    /// 是否成功
    pub success: bool,
    /// 删除的数量
    pub deleted_count: i32,
}