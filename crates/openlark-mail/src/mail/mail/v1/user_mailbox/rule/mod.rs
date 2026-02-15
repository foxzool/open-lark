pub mod create;
pub mod delete;
pub mod list;
pub mod reorder;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Rule {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Rule {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailboxRuleRequest {
        list::ListMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn create(&self) -> create::CreateMailboxRuleRequest {
        create::CreateMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn delete(&self, rule_id: impl Into<String>) -> delete::DeleteMailboxRuleRequest {
        delete::DeleteMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone(), rule_id)
    }

    pub fn reorder(&self) -> reorder::ReorderMailboxRuleRequest {
        reorder::ReorderMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
}
