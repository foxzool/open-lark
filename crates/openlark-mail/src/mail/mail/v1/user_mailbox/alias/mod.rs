pub mod create;
pub mod delete;
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailboxAliasRequest {
        list::ListMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn create(&self) -> create::CreateMailboxAliasRequest {
        create::CreateMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeleteMailboxAliasRequest {
        delete::DeleteMailboxAliasRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            alias_id,
        )
    }
}
