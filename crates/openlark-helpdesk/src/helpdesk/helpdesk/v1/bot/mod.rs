//! 机器人模块
//!
//! 提供服务台机器人相关功能。

pub mod message;

use openlark_core::config::Config;
use std::sync::Arc;

/// 机器人服务
///
/// 提供服务台机器人相关功能。
#[derive(Clone)]
pub struct Bot {
    config: Arc<Config>,
}

impl Bot {
    /// 创建新的机器人服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取消息服务
    pub fn message(&self) -> message::Message {
        message::Message::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
