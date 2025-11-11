#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// doc v2 - 文档v2版本API
//,
// 提供文档v2版本的功能
use openlark_core::prelude::*;
/// 文档v2版本服务
#[derive(Debug, Clone)]
pub struct DocV2Service {
    client: std::sync::Arc<LarkClient>,
}
impl DocV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}