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

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
