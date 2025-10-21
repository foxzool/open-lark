// 邮件组权限成员管理模块 - 占位符实现
use crate::core::config::Config;

pub struct MailGroupPermissionMemberService {
    pub config: Config,
}

impl MailGroupPermissionMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
