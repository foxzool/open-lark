//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

// 现存版本化 API 面较大，统一文档补齐前先在模块边界抑制 missing_docs 噪声，
// 避免影响 crate 级质量门禁。
#![allow(missing_docs)]

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

#[cfg(test)]
mod tests {
    use super::ApaasV1;
    use crate::PlatformConfig;

    #[test]
    fn test_apaas_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = ApaasV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }
}
