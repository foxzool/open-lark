use std::sync::Arc;
use openlark_core::config::Config;

#[derive(Clone)]
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
    pub fn v1(&self) -> crate::v1::HelpdeskV1 {
        crate::v1::HelpdeskV1::new(self.config.clone())
    }

    #[cfg(feature = "v1")]
    pub fn ticket(&self) -> crate::v1::ticket::Ticket {
        crate::v1::ticket::Ticket::new(self.config.clone())
    }
}
