use openlark_core::config::Config;
use std::sync::Arc;

/// Attachment：附件资源
#[derive(Clone)]
#[allow(dead_code)]
pub struct Attachment {
    config: Arc<Config>,
}

impl Attachment {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
