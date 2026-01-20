use std::sync::Arc;
use openlark_core::config::Config;

/// Section：分组资源
#[derive(Clone)]
pub struct Section {
    config: Arc<Config>,
}

impl Section {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
