//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

pub mod app;
pub mod application;
pub mod approval_instance;
pub mod approval_task;
pub mod seat_activity;
pub mod seat_assignment;
pub mod user_task;
pub mod workspace;

/// aPaaS V1 API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ApaasV1 {
    config: Arc<PlatformConfig>,
}

impl ApaasV1 {
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }
}
