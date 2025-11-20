
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// wiki v1 - 知识库v1版本API
//,
// 包含知识库的基础功能
use openlark_core::prelude::*;
/// 知识库v1版本服务
#[derive(Clone, Debug)]
pub struct WikiV1Service {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}
impl WikiV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}