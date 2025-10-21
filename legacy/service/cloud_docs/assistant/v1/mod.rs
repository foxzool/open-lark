use crate::core::config::Config;

pub use subscription::SubscriptionService;

pub mod subscription;

/// 云文档助手V1 API服务
pub struct V1 {
    /// 订阅管理
    pub subscription: SubscriptionService,
    #[allow(dead_code)]
    config: Config,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            subscription: SubscriptionService::new(config.clone()),
            config,
        }
    }
}
