//! 用户邮箱模块

pub mod alias;
pub mod event;
pub mod folder;
pub mod mail_contact;
pub mod message;
pub mod rule;

use openlark_core::config::Config;
use std::sync::Arc;

/// UserMailbox：用户邮箱资源
#[derive(Clone)]
pub struct UserMailbox {
    config: Arc<Config>,
    mailbox_id: String,
}

impl UserMailbox {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 获取别名管理
    pub fn alias(&self) -> alias::Alias {
        alias::Alias::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取事件管理
    pub fn event(&self) -> event::Event {
        event::Event::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取文件夹管理
    pub fn folder(&self) -> folder::Folder {
        folder::Folder::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取邮件联系人管理
    pub fn mail_contact(&self) -> mail_contact::MailContact {
        mail_contact::MailContact::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取消息管理
    pub fn message(&self) -> message::Message {
        message::Message::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取规则管理
    pub fn rule(&self) -> rule::Rule {
        rule::Rule::new(self.config.clone(), self.mailbox_id.clone())
    }
}
