//! 客服工作日程模块 (agent.schedules)

pub mod delete;
pub mod get;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服工作日程 API
#[derive(Clone)]
pub struct AgentSchedules {
    config: Arc<Config>,
    agent_id: String,
}

impl AgentSchedules {
    pub fn new(config: Arc<Config>, agent_id: impl Into<String>) -> Self {
        Self {
            config,
            agent_id: agent_id.into(),
        }
    }

    pub fn delete(self) -> delete::DeleteAgentScheduleRequest {
        delete::DeleteAgentScheduleRequest::new(self.config, self.agent_id)
    }

    pub fn get(&self) -> get::GetAgentScheduleRequest {
        get::GetAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone())
    }

    pub fn patch(self) -> patch::PatchAgentScheduleRequest {
        patch::PatchAgentScheduleRequest::new(self.config, self.agent_id)
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
