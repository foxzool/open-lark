#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// lingo v1 - 词典v1版本API
//,
// 包含词典的完整功能
use crate::core::config::Config;
/// 词典v1版本服务
#[derive(Debug, Clone)]
pub struct LingoV1Service {
    config: Config,
}
impl LingoV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}