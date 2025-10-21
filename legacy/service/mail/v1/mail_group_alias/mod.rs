// 邮件组别名管理模块 - 占位符实现
use crate::core::config::Config;

pub struct MailGroupAliasService {
    pub config: Config,
}

impl MailGroupAliasService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
