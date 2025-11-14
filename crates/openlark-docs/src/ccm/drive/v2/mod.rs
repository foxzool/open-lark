#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v2 - 云空间文件管理v2版本API
//,
// 包含云空间文件管理的扩展功能
use openlark_core::{config::Config, trait_system::Service};
/// 云空间文件管理v2版本服务
#[derive(Clone)]
pub struct DriveV2Service {
    client: std::sync::Arc<LarkClient>,
}
impl DriveV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}