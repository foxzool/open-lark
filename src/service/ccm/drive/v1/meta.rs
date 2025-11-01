// drive v1 meta - 元数据服务,
//,
// 提供文件元数据相关的功能,
use crate::prelude::*;
/// 元数据服务
#[derive(Debug, Clone)]
pub struct MetaService {
    client: std::sync::Arc<LarkClient>,
}
impl MetaService {
    pub fn new(config: Config) -> Self {
        Self { config }
}