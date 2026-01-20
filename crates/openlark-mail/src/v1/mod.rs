pub mod mail_group;

use std::sync::Arc;
use openlark_core::config::Config;

/// MailV1：邮件 API v1 访问入口
#[derive(Clone)]
pub struct MailV1 {
    config: Arc<Config>,
}

impl MailV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问邮件组资源
    pub fn mail_group(&self) -> mail_group::MailGroup {
        mail_group::MailGroup::new(self.config.clone())
    }
}
