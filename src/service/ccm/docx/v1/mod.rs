#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// docx v1 - 文档v1版本API
//,
// 包含文档的核心功能
use crate::prelude::*;
use crate::service::ccm::docx::v1::document::DocumentService;
use crate::service::ccm::docx::v1::block::BlockService;
use crate::service::ccm::docx::v1::comment::CommentService;
/// 文档v1版本服务
#[derive(Debug, Clone)]
pub struct DocxV1Service {
}

impl DocxV1Service {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// 文档操作服务
pub mod document;
/// 块操作服务
pub mod block;
/// 评论操作服务
pub mod comment;
}