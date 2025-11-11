#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// applications - 应用管理服务
//,
// 提供应用管理相关的功能
use openlark_core::prelude::*;
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