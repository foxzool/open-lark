//! Admin V1 module

// 现存版本化 API 面较大，统一文档补齐前先在模块边界抑制 missing_docs 噪声，
// 避免影响 crate 级质量门禁。
#![allow(missing_docs)]

use crate::PlatformConfig;
use std::sync::Arc;

pub mod admin_dept_stat;
pub mod admin_user_stat;
pub mod audit;
pub mod badge;
pub mod badge_image;
pub mod password;
pub mod users;

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

#[cfg(test)]
mod tests {
    use super::AdminV1;
    use crate::PlatformConfig;

    #[test]
    fn test_admin_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = AdminV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }
}
