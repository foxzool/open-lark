// applications - 应用管理服务
//,
// 提供应用管理相关的功能
use crate::prelude::*;
use crate::service::trust_party::applications::v1::ApplicationsV1Service;
/// 应用管理服务
#[derive(Debug, Clone)]
pub struct ApplicationsService {
}

impl ApplicationsService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}