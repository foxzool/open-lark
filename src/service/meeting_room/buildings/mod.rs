#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// buildings - 建筑物管理服务
//,
// 提供建筑物管理相关的功能
use open_lark_core::config::Config;
use crate::service::meeting_room::buildings::default::BuildingsDefaultService;
/// 建筑物管理服务
#[derive(Debug, Clone)]
pub struct BuildingsService {
    /// default版本API服务
pub default: BuildingsDefaultService,
}
impl BuildingsService {
pub fn new(config: Config) -> Self {
        Self {
            default: BuildingsDefaultService::new(config),
}
/// default版本API
pub mod default;
}
}