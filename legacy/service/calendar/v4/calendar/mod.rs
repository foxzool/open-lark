use crate::{core::config::Config, impl_full_service};

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

// Service 抽象接入：Calendar v4 CalendarManagementService
impl_full_service!(CalendarManagementService, "calendar.management", "v4");

impl CalendarManagementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
