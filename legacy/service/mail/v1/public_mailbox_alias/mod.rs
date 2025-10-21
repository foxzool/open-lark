// 公共邮箱别名管理模块 - 占位符实现
use crate::core::config::Config;

pub struct PublicMailboxAliasService {
    pub config: Config,
}

impl PublicMailboxAliasService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
