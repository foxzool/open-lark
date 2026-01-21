use openlark_core::config::Config;
use std::sync::Arc;

/// Tasklist：任务清单资源
#[derive(Clone)]
#[allow(dead_code)]
pub struct Tasklist {
    config: Arc<Config>,
}

impl Tasklist {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
