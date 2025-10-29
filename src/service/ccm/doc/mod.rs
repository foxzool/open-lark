// doc - 文档服务
//
// 提供文档相关的功能

use crate::prelude::*;
use crate::service::ccm::doc::v2::DocV2Service;

/// 文档服务
#[derive(Debug, Clone)]
pub struct DocService {
    /// v2版本API服务
    pub v2: DocV2Service,
}

impl DocService {
    /// 创建新的文档服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            v2: DocV2Service::new(client.clone()),
        }
    }
}

/// v2版本API
pub mod v2;