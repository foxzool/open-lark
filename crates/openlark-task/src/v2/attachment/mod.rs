use std::sync::Arc;
use openlark_core::config::Config;

/// Attachment：附件资源
#[derive(Clone)]
pub struct Attachment {
    config: Arc<Config>,
}

impl Attachment {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
