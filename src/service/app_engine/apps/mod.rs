// apps - 应用管理服务
//,
// 提供应用管理相关的功能
use crate::prelude::*;
use crate::service::app_engine::apps::v1::AppsV1Service;
/// 应用管理服务
#[derive(Debug, Clone)]
pub struct AppsService {
}

impl AppsService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}