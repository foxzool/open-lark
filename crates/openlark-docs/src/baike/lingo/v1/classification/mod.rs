/// 词典分类管理模块
///
/// 提供词典分类的查询功能。
use openlark_core::config::Config;

// 导出具体的API实现
pub mod list;

// 重新导出API函数
// list 模块显式导出
pub use list::{
    ListClassificationRequest,
    ListClassificationResp,
    execute,
    execute_with_options,
    new,
    page_size,
    page_token,
    repo_id,
};

/// 词典分类服务
#[derive(Debug, Clone)]
pub struct ClassificationService {
    config: Config,
}

impl ClassificationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for ClassificationService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
