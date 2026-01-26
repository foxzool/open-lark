use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod highlight;
pub mod list;
pub mod r#match;
pub mod search;
pub mod update;

// 使用通配符导出所有子模块
pub use create::*;
pub use delete::*;
pub use get::*;
pub use highlight::*;
pub use list::*;
pub use r#match::*;
pub use search::*;
pub use update::*;

/// Lingo词条管理服务
#[derive(Debug, Clone)]
pub struct LingoEntityService {
    config: Config,
}

impl LingoEntityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for LingoEntityService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
