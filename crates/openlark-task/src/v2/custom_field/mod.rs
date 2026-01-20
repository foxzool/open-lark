use std::sync::Arc;
use openlark_core::config::Config;

/// CustomField：自定义字段资源
#[derive(Clone)]
pub struct CustomField {
    config: Arc<Config>,
}

impl CustomField {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
