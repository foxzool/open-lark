use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod get;

/// Exchange绑定服务
pub struct ExchangeBindingService {
    pub config: Config,
}

impl ExchangeBindingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
