
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets v2 style - 样式操作API,
//,
// 实现单元格样式设置的操作,
use openlark_core::prelude::*;
/// 样式操作服务
#[derive(Clone, Debug)]
pub struct StyleService {
    #[allow(dead_code)]
    config: Config,,
}
impl StyleService {
    pub fn new(config: Config) -> Self {
        Self { config }
}