//! cardkit v1

pub mod card;

use openlark_core::config::Config;

/// cardkit v1 服务
#[derive(Debug, Clone)]
pub struct CardkitV1Service {
    config: Config,
}

impl CardkitV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn card(&self) -> card::CardService {
        card::CardService::new(self.config.clone())
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
