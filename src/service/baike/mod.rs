// baike - 企业词典功能模块
//
// 该模块提供飞书企业词典相关的所有功能，包括：
// - 词条管理（创建、更新、删除、查询）
// - 草稿管理（创建、编辑、提交审核）
// - 词典搜索（精准搜索、模糊搜索）
// - 词条高亮和分类管理
// - 审核流程管理
//
// 覆盖27个API接口，是企业知识管理的重要组成部分

use crate::core::config::Config;
use crate::service::baike::lingo::LingoService;
use crate::service::baike::dictionary::DictionaryService;

/// 企业词典功能服务
#[cfg(feature = "baike")]
#[derive(Debug, Clone)]
pub struct BaikeService {
    /// 词典服务
    pub lingo: LingoService,
    /// 词典管理服务
    pub dictionary: DictionaryService,
}

#[cfg(feature = "baike")]
impl BaikeService {
    /// 创建新的企业词典功能服务实例
    pub fn new(config: Config) -> Self {
        Self {
            lingo: LingoService::new(config.clone()),
            dictionary: DictionaryService::new(config.clone()),
        }
    }
}

#[cfg(not(feature = "baike"))]
pub struct BaikeService;

/// 数据模型
pub mod models;

/// 各子模块
pub mod lingo;
pub mod dictionary;