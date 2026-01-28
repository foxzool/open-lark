//! 退回人工任务
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 退回人工任务 Builder
#[derive(Debug, Clone)]
pub struct RollbackTaskBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 退回节点 ID
    node_id: String,
    /// 退回原因
    reason: Option<String>,
}

impl RollbackTaskBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            node_id: node_id.into(),
            reason: None,
        }
    }

    /// 设置退回原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RollbackTaskResponse> {
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/rollback", self.task_id);

        let request = RollbackTaskRequest {
            node_id: self.node_id,
            reason: self.reason,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<RollbackTaskResponse> {
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/rollback", self.task_id);

        let request = RollbackTaskRequest {
            node_id: self.node_id,
            reason: self.reason,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 退回请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RollbackTaskRequest {
    /// 退回节点 ID
    #[serde(rename = "node_id")]
    node_id: String,
    /// 退回原因
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// 退回响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackTaskResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    task_id: String,
    /// 退回结果
    #[serde(rename = "result")]
    result: bool,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for RollbackTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
