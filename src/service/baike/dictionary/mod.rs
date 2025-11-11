#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// dictionary - 词典管理服务
//,
// 提供词典管理相关的功能
use open_lark_core::config::Config;
use crate::service::baike::dictionary::v1::DictionaryV1Service;
/// 词典管理服务
#[derive(Debug, Clone)]
pub struct DictionaryService {
}

impl DictionaryService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}