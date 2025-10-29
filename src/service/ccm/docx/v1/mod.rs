// docx v1 - 文档v1版本API
//
// 包含文档的核心功能

use crate::prelude::*;
use crate::service::ccm::docx::v1::document::DocumentService;
use crate::service::ccm::docx::v1::block::BlockService;
use crate::service::ccm::docx::v1::comment::CommentService;

/// 文档v1版本服务
#[derive(Debug, Clone)]
pub struct DocxV1Service {
    client: std::sync::Arc<LarkClient>,
    /// 文档操作服务
    pub document: DocumentService,
    /// 块操作服务
    pub block: BlockService,
    /// 评论操作服务
    pub comment: CommentService,
}

impl DocxV1Service {
    /// 创建新的v1版本服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            document: DocumentService::new(client.clone()),
            block: BlockService::new(client.clone()),
            comment: CommentService::new(client.clone()),
        }
    }
}

/// 文档操作服务
pub mod document;
/// 块操作服务
pub mod block;
/// 评论操作服务
pub mod comment;