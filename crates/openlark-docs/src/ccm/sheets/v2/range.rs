
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets v2 range - 范围操作API,
//,
// 实现单元格范围的操作,
use openlark_core::prelude::*;
/// 范围操作服务
#[derive(Clone, Debug)]
pub struct RangeService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}
impl RangeService {
    pub fn new(config: Config) -> Self {
        Self { config }
}