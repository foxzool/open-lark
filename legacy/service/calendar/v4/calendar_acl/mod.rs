use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod list;
pub mod subscription;
pub mod unsubscription;

// 重新导出所有请求和响应类型

/// 日历访问控制服务
///
/// 提供日历访问控制的管理功能
pub struct CalendarAclService {
    pub config: Config,
}

impl CalendarAclService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
