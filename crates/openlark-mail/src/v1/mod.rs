pub mod mail_group;
pub mod public_mailbox;
pub mod user;
pub mod user_mailbox;

use openlark_core::config::Config;
use std::sync::Arc;

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

    /// 访问公共邮箱资源
    pub fn public_mailbox(&self) -> public_mailbox::PublicMailbox {
        public_mailbox::PublicMailbox::new(self.config.clone())
    }

    /// 访问用户邮箱资源
    pub fn user(&self) -> user::User {
        user::User::new(self.config.clone())
    }

    /// 访问用户邮箱资源
    pub fn user_mailbox(&self, mailbox_id: impl Into<String>) -> user_mailbox::UserMailbox {
        user_mailbox::UserMailbox::new(self.config.clone(), mailbox_id.into())
    }
}
