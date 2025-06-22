use std::sync::Arc;

pub mod v2;
pub mod v3;

pub struct SheetsService {
    pub v2: v2::V2,
    pub v3: v3::V3,
}

impl SheetsService {
    pub fn new(config: Arc<crate::core::config::Config>) -> Self {
        Self {
            v2: v2::V2::new((*config).clone()),
            v3: v3::V3::new((*config).clone()),
        }
    }
}
