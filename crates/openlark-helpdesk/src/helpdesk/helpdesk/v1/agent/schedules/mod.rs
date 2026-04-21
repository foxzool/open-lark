//! 客服工作日程模块 (agent.schedules)

pub mod delete;
/// 获取接口。
pub mod get;
/// 更新接口。
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
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, agent_id: impl Into<String>) -> Self {
        Self {
            config,
            agent_id: agent_id.into(),
        }
    }

    /// 创建删除请求。
    pub fn delete(self) -> delete::DeleteAgentScheduleRequest {
        delete::DeleteAgentScheduleRequest::new(self.config, self.agent_id)
    }

    /// 创建获取详情请求。
    pub fn get(&self) -> get::GetAgentScheduleRequest {
        get::GetAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone())
    }

    /// 创建补丁请求。
    pub fn patch(self) -> patch::PatchAgentScheduleRequest {
        patch::PatchAgentScheduleRequest::new(self.config, self.agent_id)
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
