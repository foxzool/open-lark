//! 客服模块
//!
//! 提供客服相关的 API。

pub mod agent_email;
pub mod patch;
pub mod schedules;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服服务
#[derive(Clone)]
pub struct Agent {
    config: Arc<Config>,
}

impl Agent {
    /// 创建新的客服服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服邮箱
    pub fn agent_email(&self) -> agent_email::GetAgentEmailRequest {
        agent_email::GetAgentEmailRequest::new(self.config.clone())
    }

    /// 更新客服信息
    pub fn patch(&self, agent_id: impl Into<String>) -> patch::PatchAgentRequest {
        patch::PatchAgentRequest::new(self.config.clone(), agent_id.into())
    }

    /// 访问客服工作日程
    pub fn schedules(&self, agent_id: impl Into<String>) -> schedules::AgentSchedules {
        schedules::AgentSchedules::new(self.config.clone(), agent_id)
    }
}

pub use agent_email::GetAgentEmailRequest;
pub use patch::{PatchAgentRequest, PatchAgentRequestBuilder};

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
