/// CCM Doc API Old V1 数据模型
///
/// 包含所有旧版文档相关的数据结构定义

use serde::{Deserialize, Serialize};

/// 文档类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    /// 文档
    #[serde(rename = "doc")]
    Doc,
    /// 电子表格
    #[serde(rename = "sheet")]
    Sheet,
    /// 多维表格
    #[serde(rename = "bitable")]
    Bitable,
    /// 思维笔记
    #[serde(rename = "mindnote")]
    Mindnote,
    /// 文件
    #[serde(rename = "file")]
    File,
}

/// 更新操作类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateOperationType {
    /// 插入块
    #[serde(rename = "insert_block")]
    InsertBlock,
    /// 更新块
    #[serde(rename = "update_block")]
    UpdateBlock,
    /// 删除块
    #[serde(rename = "delete_block")]
    DeleteBlock,
    /// 更新标题
    #[serde(rename = "update_title")]
    UpdateTitle,
}