pub mod subscribe;
pub mod unsubscribe;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Event {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Event {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn subscribe(&self) -> subscribe::SubscribeMailboxEventRequest {
        subscribe::SubscribeMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn unsubscribe(&self) -> unsubscribe::UnsubscribeMailboxEventRequest {
        unsubscribe::UnsubscribeMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
}
