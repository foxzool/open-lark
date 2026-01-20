use std::sync::Arc;
use openlark_core::{config::Config, validate_required, SDKResult};
use serde::Serialize;

/// 创建任务请求体（v1）
#[derive(Debug, Clone, Serialize)]
pub struct CreateTaskBodyV1 {
    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,

    /// 任务优先级（1-5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 创建任务请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskRequestV1 {
    config: Arc<Config>,
    body: CreateTaskBodyV1,
}

impl CreateTaskRequestV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTaskBodyV1 {
                summary: String::new(),
                description: None,
                start: None,
                due: None,
                priority: None,
            },
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.body.summary.trim(), "任务标题不能为空");

        let endpoint = "/open-apis/task/v1/tasks";
        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| openlark_core::error::validation_error("序列化请求体失败", &e.to_string()))?;

        let response = openlark_core::http::Transport::request(
            openlark_core::api::ApiRequest::<serde_json::Value>::post(endpoint).body(body_json),
            &self.config,
            None,
        )
        .await?;

        response.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}
