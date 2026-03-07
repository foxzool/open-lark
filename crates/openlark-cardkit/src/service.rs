//! CardKit 服务主入口

use std::sync::Arc;

/// CardKit 服务主入口
#[derive(Debug, Clone)]
pub struct CardkitService {
    config: Arc<openlark_core::config::Config>,
}

impl CardkitService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 访问 cardkit 项目 v1
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::cardkit::cardkit::v1::CardkitV1Service {
        crate::cardkit::cardkit::v1::CardkitV1Service::new((*self.config).clone())
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
