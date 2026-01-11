//! 执行数据知识问答
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/ask

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, SDKResult,
};
use openlark_core::validate_required;
use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_KNOWLEDGE_ASK};

/// 执行数据知识问答请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AskKnowledgeBody {
    pub question: String,
    pub stream: Option<bool>,
}

/// 执行数据知识问答请求
pub struct AskKnowledgeRequest {
    config: Config,
    app_id: String,
}

impl AskKnowledgeRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    pub async fn execute(
        self,
        body: AskKnowledgeBody,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_KNOWLEDGE_ASK.replace("{app_id}", &self.app_id);
        let req: ApiRequest<AskKnowledgeBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "执行数据知识问答")
    }
}
