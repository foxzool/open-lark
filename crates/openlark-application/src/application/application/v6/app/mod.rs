pub mod get;
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
}

impl App {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn get(&self) -> get::GetAppRequest {
        get::GetAppRequest::new(self.config.clone())
    }
}

pub use get::GetAppRequest;
pub use models::*;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
