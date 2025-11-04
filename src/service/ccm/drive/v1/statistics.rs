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
use crate::prelude::*;
/// 统计服务
#[derive(Debug, Clone)]
pub struct StatisticsService {
    client: std::sync::Arc<LarkClient>,
}
impl StatisticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}