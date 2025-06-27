// 公共邮箱管理模块 - 占位符实现
use crate::core::config::Config;

pub struct PublicMailboxService {
    pub config: Config,
}

impl PublicMailboxService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
