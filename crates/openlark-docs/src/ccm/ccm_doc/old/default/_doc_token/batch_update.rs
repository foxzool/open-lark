//! 编辑旧版文档内容
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocRequest {
    pub revision: i32,
    pub requests: Vec<DocRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DocRequest {
    /// 文档编辑请求结构较复杂，允许按文档示例透传 JSON。
    #[serde(flatten)]
    pub request: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocResponse {}

impl ApiResponseTrait for BatchUpdateDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量编辑更新旧版文档内容请求
pub struct BatchUpdateDocRequestBuilder {
    config: Config,
    doc_token: String,
    req: BatchUpdateDocRequest,
}

impl BatchUpdateDocRequestBuilder {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
            req: BatchUpdateDocRequest::default(),
        }
    }

    pub fn revision(mut self, revision: i32) -> Self {
        self.req.revision = revision;
        self
    }

    pub fn requests(mut self, requests: Vec<DocRequest>) -> Self {
        self.req.requests = requests;
        self
    }

    pub async fn send(self) -> SDKResult<BatchUpdateDocResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<BatchUpdateDocResponse> =
            ApiRequest::post(&CcmDocApiOld::BatchUpdate(self.doc_token).to_url())
                .body(serde_json::to_value(&self.req)?);
        let response: Response<BatchUpdateDocResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
