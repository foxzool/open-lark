use std::sync::Arc;
use openlark_core::config::Config;

/// Tasklist：任务清单资源
#[derive(Clone)]
pub struct Tasklist {
    config: Arc<Config>,
}

impl Tasklist {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
