use std::sync::Arc;

use crate::core::config::Config;

pub mod v1;

pub struct SearchService {
    pub v1: v1::V1,
}

impl SearchService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            v1: v1::V1::new((*config).clone()),
        }
    }
}
