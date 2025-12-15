/// Lingo词条管理模块
///
/// 提供Lingo语言服务词条的创建、更新、删除、查询等功能。

use openlark_core::config::Config;

// 导出具体的API实现
pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub mod list;
pub mod r#match;
pub mod search;
pub mod highlight;
pub mod batch_get;
pub mod batch_update;
pub mod search_recommend;
pub mod history_get;
pub mod history_list;

// 重新导出API函数
pub use create::*;
pub use update::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use r#match::*;
pub use search::*;
pub use highlight::*;
pub use batch_get::*;
pub use batch_update::*;
pub use search_recommend::*;
pub use history_get::*;
pub use history_list::*;

/// Lingo词条管理服务
#[derive(Debug, Clone)]
pub struct LingoEntityService {
    config: Config,
}

impl LingoEntityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for LingoEntityService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}