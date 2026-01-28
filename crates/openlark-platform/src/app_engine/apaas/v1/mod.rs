//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

pub mod app;
pub mod approval_instance;
pub mod approval_task;

use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

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
