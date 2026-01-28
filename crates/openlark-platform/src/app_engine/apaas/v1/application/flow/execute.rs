//! 发起流程
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/application-flow/execute

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 发起流程 Builder
#[derive(Debug, Clone)]
pub struct FlowExecuteBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 流程 ID
    flow_id: String,
    /// 流程参数
    params: serde_json::Value,
}

impl FlowExecuteBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        flow_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            flow_id: flow_id.into(),
            params: serde_json::json!({}),
        }
    }

    /// 设置流程参数
    pub fn params(mut self, params: impl Into<serde_json::Value>) -> Self {
        self.params = params.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FlowExecuteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/flows/{}/execute",
            self.namespace, self.flow_id
        );

        let request = FlowExecuteRequest {
            params: self.params,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<FlowExecuteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/flows/{}/execute",
            self.namespace, self.flow_id
        );

        let request = FlowExecuteRequest {
            params: self.params,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 发起流程请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct FlowExecuteRequest {
    /// 流程参数
    #[serde(rename = "params")]
    params: serde_json::Value,
}

/// 发起流程响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlowExecuteResponse {
    /// 实例 ID
    #[serde(rename = "instance_id")]
    instance_id: String,
    /// 流程状态
    #[serde(rename = "status")]
    status: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for FlowExecuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
