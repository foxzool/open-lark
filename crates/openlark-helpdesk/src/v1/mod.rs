pub mod ticket;

use std::sync::Arc;
use openlark_core::config::Config;

#[derive(Clone)]
pub struct HelpdeskV1 {
    config: Arc<Config>,
}

impl HelpdeskV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn ticket(&self) -> ticket::Ticket {
        ticket::Ticket::new(self.config.clone())
    }
}
