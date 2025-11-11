#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// rooms default - 会议室管理default版本API
//,
// 包含会议室管理的完整功能
use config::Config;
/// 会议室管理default版本服务
#[derive(Debug, Clone)]
pub struct RoomsDefaultService {
    config: Config,
}
impl RoomsDefaultService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}