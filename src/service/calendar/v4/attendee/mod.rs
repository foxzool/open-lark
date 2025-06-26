use crate::core::config::Config;

pub mod batch_delete;
pub mod create;
pub mod list;
pub mod list_chat_members;


/// 参与人管理服务
pub struct AttendeeService {
    pub config: Config,
}

impl AttendeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
