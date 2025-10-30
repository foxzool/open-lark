// docx v1 block - 块操作API,
//,
// 实现文档块的操作,
use crate::prelude::*;
/// 块操作服务
#[derive(Debug, Clone)]
pub struct BlockService {
    client: std::sync::Arc<LarkClient>,
}
impl BlockService {
    pub fn new(config: Config) -> Self {
        Self { config }
}