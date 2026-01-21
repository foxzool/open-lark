use openlark_core::config::Config;
use std::sync::Arc;

/// Comment：评论资源
#[derive(Clone)]
#[allow(dead_code)]
pub struct Comment {
    config: Arc<Config>,
}

impl Comment {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
