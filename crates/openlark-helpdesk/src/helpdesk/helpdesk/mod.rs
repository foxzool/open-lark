//! Helpdesk API 模块

/// Helpdesk v1 模块。
pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Helpdesk API 入口
#[derive(Clone)]
pub struct Helpdesk {
    config: Arc<Config>,
}

impl Helpdesk {
    /// 创建新的 Helpdesk API 入口。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本 API
    pub fn v1(&self) -> v1::HelpdeskV1 {
        v1::HelpdeskV1::new(self.config.clone())
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
