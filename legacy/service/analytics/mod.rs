pub use v1::*;

mod v1;

/// 高级业务分析服务
pub struct AnalyticsService {
    config: crate::core::config::Config,
}

impl AnalyticsService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self { config }
    }
}

impl crate::core::service_traits::Service for AnalyticsService {
    fn config(&self) -> &crate::core::config::Config {
        &self.config
    }
}