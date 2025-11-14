#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive explorer - 资源浏览器API
//,
// 提供云空间资源浏览器相关的功能
use openlark_core::{config::Config, trait_system::Service};
/// 资源浏览器服务
#[derive(Clone)]
pub struct DriveExplorerService {
    client: std::sync::Arc<LarkClient>,
}
impl DriveExplorerService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}