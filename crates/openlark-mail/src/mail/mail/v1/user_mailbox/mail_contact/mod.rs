pub mod create;
pub mod delete;
pub mod list;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct MailContact {
    config: Arc<Config>,
    mailbox_id: String,
}

impl MailContact {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailContactRequest {
        list::ListMailContactRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn create(&self) -> create::CreateMailContactRequest {
        create::CreateMailContactRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn patch(&self, mail_contact_id: impl Into<String>) -> patch::PatchMailContactRequest {
        patch::PatchMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            mail_contact_id,
        )
    }

    pub fn delete(&self, mail_contact_id: impl Into<String>) -> delete::DeleteMailContactRequest {
        delete::DeleteMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            mail_contact_id,
        )
    }
}
