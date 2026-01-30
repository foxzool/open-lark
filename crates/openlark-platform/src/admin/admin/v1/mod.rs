//! Admin V1 module

use crate::PlatformConfig;
use std::sync::Arc;

pub mod users;
pub mod admin_dept_stat;
pub mod admin_user_stat;
pub mod audit;
pub mod badge;
pub mod badge_image;
pub mod password;

/// Admin V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AdminV1 {
    config: Arc<PlatformConfig>,
}

impl AdminV1 {
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}
