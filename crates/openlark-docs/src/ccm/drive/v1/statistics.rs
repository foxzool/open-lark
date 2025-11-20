
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v1 statistics - 统计服务,
//,
// 提供文件统计相关的功能,
use openlark_core::{config::Config, trait_system::Service};
/// 统计服务
#[derive(Clone, Debug)]
pub struct StatisticsService {
    #[allow(dead_code)]
    config: Config,,
}
impl StatisticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}