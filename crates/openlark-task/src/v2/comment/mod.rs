use std::sync::Arc;
use openlark_core::config::Config;

/// Comment：评论资源
#[derive(Clone)]
pub struct Comment {
    config: Arc<Config>,
}

impl Comment {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
