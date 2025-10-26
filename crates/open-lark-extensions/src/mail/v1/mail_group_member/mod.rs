// 邮件组成员管理模块 - 占位符实现
use open_lark_core::core::config::Config;

pub struct MailGroupMemberService {
    pub config: Config,
}

impl MailGroupMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
