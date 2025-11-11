// 用户邮箱别名管理模块 - 占位符实现
use openlark_core::config::Config;

pub struct UserMailboxAliasService {
    pub config: Config,
}

impl UserMailboxAliasService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
