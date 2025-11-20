
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v1 meta - 元数据服务,
//,
// 提供文件元数据相关的功能,
use openlark_core::{config::Config, trait_system::Service};
/// 元数据服务
#[derive(Clone, Debug)]
pub struct MetaService {
    #[allow(dead_code)]
    config: Config,,
}
impl MetaService {
    pub fn new(config: Config) -> Self {
        Self { config }
}