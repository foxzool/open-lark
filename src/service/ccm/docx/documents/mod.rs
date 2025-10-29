// docx documents - 文档操作API
//
// 提供文档操作的功能

use crate::prelude::*;

/// 文档操作服务
#[derive(Debug, Clone)]
pub struct DocumentsService {
    client: std::sync::Arc<LarkClient>,
}

impl DocumentsService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}