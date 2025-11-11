#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// apps v1 - 应用管理v1版本API
//,
// 包含应用管理的完整功能
use openlark_core::prelude::*;
/// 应用管理v1版本服务
#[derive(Debug, Clone)]
pub struct AppsV1Service {
    client: std::sync::Arc<crate::client::LarkClient>,
}
impl AppsV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}