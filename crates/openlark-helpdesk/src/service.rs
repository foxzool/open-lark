use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct HelpdeskService {
    config: Arc<Config>,
}

impl HelpdeskService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn helpdesk(&self) -> crate::helpdesk::helpdesk::Helpdesk {
        crate::helpdesk::helpdesk::Helpdesk::new(self.config.clone())
    }

    #[cfg(feature = "v1")]
    pub fn ticket(&self) -> crate::helpdesk::helpdesk::v1::ticket::Ticket {
        crate::helpdesk::helpdesk::v1::ticket::Ticket::new(self.config.clone())
    }
}
