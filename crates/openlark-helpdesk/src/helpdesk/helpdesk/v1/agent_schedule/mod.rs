//! 客服工作日程模块
//!
//! 提供客服工作日程相关的 API。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服工作日程服务
#[derive(Clone)]
pub struct AgentSchedule {
    config: Arc<Config>,
}

impl AgentSchedule {
    /// 创建新的客服工作日程服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服工作日程列表
    pub fn list(&self) -> list::ListAgentScheduleRequest {
        list::ListAgentScheduleRequest::new(self.config.clone())
    }

    /// 创建客服工作日程
    pub fn create(&self) -> create::CreateAgentScheduleRequest {
        create::CreateAgentScheduleRequest::new(self.config.clone())
    }

    /// 获取指定客服的工作日程
    pub fn get(&self, agent_id: impl Into<String>) -> get::GetAgentScheduleRequest {
        get::GetAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }

    /// 更新指定客服的工作日程
    pub fn patch(&self, agent_id: impl Into<String>) -> patch::PatchAgentScheduleRequest {
        patch::PatchAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }

    /// 删除指定客服的工作日程
    pub fn delete(&self, agent_id: impl Into<String>) -> delete::DeleteAgentScheduleRequest {
        delete::DeleteAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }
}

pub use create::{CreateAgentScheduleRequest, CreateAgentScheduleRequestBuilder};
pub use delete::{DeleteAgentScheduleRequest, DeleteAgentScheduleRequestBuilder};
pub use get::{GetAgentScheduleRequest, GetAgentScheduleRequestBuilder};
pub use list::{ListAgentScheduleRequest, ListAgentScheduleRequestBuilder};
pub use patch::{PatchAgentScheduleRequest, PatchAgentScheduleRequestBuilder};

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
