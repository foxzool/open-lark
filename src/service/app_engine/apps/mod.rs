// apps - 应用管理服务
//,
// 提供应用管理相关的功能
use crate::prelude::*;
use crate::service::app_engine::apps::v1::AppsV1Service;
/// 应用管理服务
#[derive(.*?)]
pub struct AppsService {
    /// v1版本API服务
    pub v1: AppsV1Service,
}
impl AppsService {
    /// 创建新的应用管理服务实例
pub fn new() -> Self {
        Self {
            v1: AppsV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;