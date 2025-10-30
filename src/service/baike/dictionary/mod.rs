// dictionary - 词典管理服务
//,
// 提供词典管理相关的功能
use crate::core::config::Config;
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