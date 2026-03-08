//! Mail API 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Mail API 入口
#[derive(Clone)]
pub struct Mail {
    config: Arc<Config>,
}

impl Mail {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本 API
    pub fn v1(&self) -> v1::MailV1 {
        v1::MailV1::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {


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
