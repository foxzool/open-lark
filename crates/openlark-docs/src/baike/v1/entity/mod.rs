/// 词条管理模块
///
/// 提供知识库词条的创建、更新、查询、删除、审批等功能。

use openlark_core::config::Config;

// 导出具体的API实现
pub mod create;
pub mod update;
pub mod get;
pub mod delete;
pub mod list;
pub mod r#match;
pub mod search;
pub mod highlight;
pub mod extract;
pub mod approve;
pub mod reject;
pub mod audit_list;

// 重新导出API函数
pub use create::*;
pub use update::*;
pub use get::*;
pub use delete::*;
pub use list::*;
pub use r#match::*;
pub use search::*;
pub use highlight::*;
pub use extract::*;
pub use approve::*;
pub use reject::*;
pub use audit_list::*;

/// 词条管理服务
#[derive(Debug, Clone)]
pub struct EntityService {
    config: Config,
}

impl EntityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for EntityService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}