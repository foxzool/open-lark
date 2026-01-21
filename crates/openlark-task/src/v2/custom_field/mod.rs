use openlark_core::config::Config;
use std::sync::Arc;

/// CustomField：自定义字段资源
#[derive(Clone)]
#[allow(dead_code)]
pub struct CustomField {
    config: Arc<Config>,
}

impl CustomField {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
