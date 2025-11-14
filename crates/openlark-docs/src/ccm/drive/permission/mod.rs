#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// permission - 文档权限管理API
//,
// 提供文档权限相关的功能
use openlark_core::prelude::*;
/// 文档权限管理服务
#[derive(Clone)]
pub struct PermissionService {
    client: std::sync::Arc<LarkClient>,
}
impl PermissionService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}