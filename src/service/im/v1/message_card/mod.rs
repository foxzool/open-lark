use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod patch;
// pub mod delay_update;
// pub mod send_visible;
// pub mod delete_visible;

/// 消息卡片服务
///
/// 提供消息卡片的更新、延时更新、可见性控制等功能
pub struct MessageCardService {
    pub config: Config,
}

impl MessageCardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}