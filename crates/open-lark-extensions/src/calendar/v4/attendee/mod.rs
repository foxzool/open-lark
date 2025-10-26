use crate::{core::config::Config, impl_full_service};

pub mod batch_delete;
pub mod create;
pub mod list;
pub mod list_chat_members;

/// 参与人管理服务
pub struct AttendeeService {
    pub config: Config,
}

// Service 抽象接入：Calendar v4 AttendeeService
impl_full_service!(AttendeeService, "calendar.attendee", "v4");

impl AttendeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
