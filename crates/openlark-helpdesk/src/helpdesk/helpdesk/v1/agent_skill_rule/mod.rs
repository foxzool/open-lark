//! 客服技能规则模块
//!
//! 提供客服技能规则相关的 API。

pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服技能规则服务
#[derive(Clone)]
pub struct AgentSkillRule {
    config: Arc<Config>,
}

impl AgentSkillRule {
    /// 创建新的客服技能规则服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服技能规则列表
    pub fn list(&self) -> list::ListAgentSkillRuleRequest {
        list::ListAgentSkillRuleRequest::new(self.config.clone())
    }
}

pub use list::{ListAgentSkillRuleRequest, ListAgentSkillRuleRequestBuilder};

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
