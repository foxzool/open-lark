// applications - 应用管理服务
//,
// 提供应用管理相关的功能
use crate::prelude::*;
use crate::service::trust_party::applications::v1::ApplicationsV1Service;
/// 应用管理服务
#[derive(.*?)]
pub struct ApplicationsService {
    /// v1版本API服务
    pub v1: ApplicationsV1Service,
}
impl ApplicationsService {
    /// 创建新的应用管理服务实例
pub fn new() -> Self {
        Self {
            v1: ApplicationsV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;