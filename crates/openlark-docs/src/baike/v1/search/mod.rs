/// 搜索模块
///
/// 提供知识库内容搜索功能，包括词条搜索、用户搜索、历史记录等。

use openlark_core::config::Config;

// 导出具体的API实现
pub mod search_entity;
pub mod search_user;
pub mod search_history;
pub mod search_history_delete;
pub mod search_entity_history;
pub mod search_space;
pub mod search_space_member;
pub mod search_space_access;
pub mod search_space_access_list;
pub mod search_space_recommend;
pub mod search_space_member_list;
pub mod search_space_operation;
pub mod search_space_operation_log;
pub mod search_space_operation_status;

// 重新导出API函数
pub use search_entity::*;
pub use search_user::*;
pub use search_history::*;
pub use search_history_delete::*;
pub use search_entity_history::*;
pub use search_space::*;
pub use search_space_member::*;
pub use search_space_access::*;
pub use search_space_access_list::*;
pub use search_space_recommend::*;
pub use search_space_member_list::*;
pub use search_space_operation::*;
pub use search_space_operation_log::*;
pub use search_space_operation_status::*;

/// 搜索服务
#[derive(Debug, Clone)]
pub struct SearchService {
    config: Config,
}

impl SearchService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}