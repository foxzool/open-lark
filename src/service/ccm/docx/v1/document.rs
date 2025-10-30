// docx v1 document - 文档操作API,
//,
// 实现文档级别的操作,
use crate::prelude::*;
/// 文档操作服务
#[derive(Debug, Clone)]
pub struct DocumentService {
    client: std::sync::Arc<LarkClient>,
}
impl DocumentService {
    pub fn new(config: Config) -> Self {
        Self { config }
}