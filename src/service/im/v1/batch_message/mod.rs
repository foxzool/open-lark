use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod send;
// pub mod delete;
// pub mod read_user;
// pub mod get_progress;

/// 批量消息服务
///
/// 提供批量发送、批量撤回、查询进度等功能
pub struct BatchMessageService {
    pub config: Config,
}

impl BatchMessageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}