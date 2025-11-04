#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// permissions v1 - 权限管理v1版本API
//,
// 包含权限管理的完整功能
use crate::prelude::*;
/// 权限管理v1版本服务
#[derive(Debug, Clone)]
pub struct PermissionsV1Service {
    client: std::sync::Arc<crate::client::LarkClient>,
}
impl PermissionsV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}