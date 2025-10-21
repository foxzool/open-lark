// 公共邮箱成员管理模块 - 占位符实现
use crate::core::config::Config;

pub struct PublicMailboxMemberService {
    pub config: Config,
}

impl PublicMailboxMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
