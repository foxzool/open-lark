use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;
pub mod primary;
pub mod search;
pub mod subscribe;
pub mod subscription;
pub mod unsubscribe;
pub mod unsubscription;

// 重新导出所有请求和响应类型
pub use create::*;
pub use get::*;
pub use list::*;

/// 日历管理服务
///
/// 提供日历的创建、删除、查询、更新、搜索等基础管理功能
pub struct CalendarManagementService {
    pub config: Config,
}

impl CalendarManagementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
