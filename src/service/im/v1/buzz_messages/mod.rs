use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod urgent_app;
// pub mod urgent_sms;
// pub mod urgent_phone;

/// 消息加急服务
///
/// 提供应用内加急、短信加急、电话加急功能
pub struct BuzzMessagesService {
    pub config: Config,
}

impl BuzzMessagesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}