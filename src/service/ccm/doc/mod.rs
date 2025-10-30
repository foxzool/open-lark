// doc - 文档服务
//,
// 提供文档相关的功能
use crate::prelude::*;
use crate::service::ccm::doc::v2::DocV2Service;
/// 文档服务
#[derive(Debug, Clone)]
pub struct DocService {
}

impl DocService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v2版本API
pub mod v2;
}