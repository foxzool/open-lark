#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// dictionary v1 - 词典管理v1版本API
//,
// 包含词典管理的完整功能
use config::Config;
/// 词典管理v1版本服务
#[derive(Debug, Clone)]
pub struct DictionaryV1Service {
    config: Config,
}
impl DictionaryV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}