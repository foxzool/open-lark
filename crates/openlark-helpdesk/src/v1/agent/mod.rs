//! 客服模块
//!
//! 提供客服相关的 API。

pub mod agent_email;
pub mod patch;

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
}

pub use agent_email::GetAgentEmailRequest;
pub use patch::{PatchAgentRequest, PatchAgentRequestBuilder};
