// lingo - 词典服务
//,
// 提供词典相关的功能，包括词条管理、草稿管理、搜索等
use crate::core::config::Config;
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