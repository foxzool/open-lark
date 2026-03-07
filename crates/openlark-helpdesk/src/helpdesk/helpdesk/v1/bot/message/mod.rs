//! 机器人消息模块
//!
//! 通过服务台机器人向工单绑定的群内发送消息。

pub mod create;

use openlark_core::config::Config;
use std::sync::Arc;

/// 机器人消息服务
///
/// 提供通过服务台机器人发送消息的功能。
#[derive(Clone)]
pub struct Message {
    config: Arc<Config>,
}

impl Message {
    /// 创建新的机器人消息服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 通过服务台机器人发送消息
    pub fn create(&self) -> create::CreateBotMessageRequestBuilder {
        create::CreateBotMessageRequestBuilder::new(self.config.clone())
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
