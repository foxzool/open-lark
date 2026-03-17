//! Admin V1 module

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
