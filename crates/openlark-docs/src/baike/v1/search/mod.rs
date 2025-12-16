/// 搜索模块
///
/// 提供知识库内容搜索功能，包括词条搜索、用户搜索、历史记录等。

use openlark_core::config::Config;

// 导出具体的API实现
// pub mod search_entity; // Generated: Module file not found
// pub mod search_user; // Generated: Module file not found
// pub mod search_history; // Generated: Module file not found
// pub mod search_history_delete; // Generated: Module file not found
// pub mod search_entity_history; // Generated: Module file not found
// pub mod search_space; // Generated: Module file not found
// pub mod search_space_member; // Generated: Module file not found
// pub mod search_space_access; // Generated: Module file not found
// pub mod search_space_access_list; // Generated: Module file not found
// pub mod search_space_recommend; // Generated: Module file not found
// pub mod search_space_member_list; // Generated: Module file not found
// pub mod search_space_operation; // Generated: Module file not found
// pub mod search_space_operation_log; // Generated: Module file not found
// pub mod search_space_operation_status; // Generated: Module file not found

// 重新导出API函数
// pub use search_entity::*; // Generated: Module use not found
// pub use search_user::*; // Generated: Module use not found
// pub use search_history::*; // Generated: Module use not found
// pub use search_history_delete::*; // Generated: Module use not found
// pub use search_entity_history::*; // Generated: Module use not found
// pub use search_space::*; // Generated: Module use not found
// pub use search_space_member::*; // Generated: Module use not found
// pub use search_space_access::*; // Generated: Module use not found
// pub use search_space_access_list::*; // Generated: Module use not found
// pub use search_space_recommend::*; // Generated: Module use not found
// pub use search_space_member_list::*; // Generated: Module use not found
// pub use search_space_operation::*; // Generated: Module use not found
// pub use search_space_operation_log::*; // Generated: Module use not found
// pub use search_space_operation_status::*; // Generated: Module use not found

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