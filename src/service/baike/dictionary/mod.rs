// dictionary - 词典管理服务
//,
// 提供词典管理相关的功能
use crate::core::config::Config;
use crate::service::baike::dictionary::v1::DictionaryV1Service;
/// 词典管理服务
#[derive(Debug, Clone)],
pub struct DictionaryService {
    /// v1版本API服务
    pub v1: DictionaryV1Service,
}
impl DictionaryService {
    /// 创建新的词典管理服务实例
pub fn new() -> Self {
        Self {
            v1: DictionaryV1Service::new(config.clone()),
        }
}
}
/// v1版本API
pub mod v1;