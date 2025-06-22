use std::sync::Arc;

pub mod v1;

pub struct BitableService {
    pub v1: v1::V1,
}

impl BitableService {
    pub fn new(config: Arc<crate::core::config::Config>) -> Self {
        Self {
            v1: v1::V1::new((*config).clone()),
        }
    }
}
