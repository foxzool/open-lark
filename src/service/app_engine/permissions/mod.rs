#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// permissions - 权限管理服务
//,
// 提供权限管理相关的功能
use crate::prelude::*;
use crate::service::app_engine::permissions::v1::PermissionsV1Service;
/// 权限管理服务
#[derive(Debug, Clone)]
pub struct PermissionsService {
}

impl PermissionsService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}