#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// lingo - 词典服务
//,
// 提供词典相关的功能，包括词条管理、草稿管理、搜索等
use open_lark_core::config::Config;
use crate::service::baike::lingo::v1::LingoV1Service;
/// 词典服务
#[derive(Debug, Clone)]
pub struct LingoService {
}

impl LingoService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}