//! 获取旧版文档纯文本内容
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocRawContentReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocRawContentResponse {
    pub content: String,
}

impl ApiResponseTrait for GetDocRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容请求
pub struct GetDocRawContentRequest {
    config: Config,
    doc_token: String,
}

impl GetDocRawContentRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<GetDocRawContentResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<GetDocRawContentResponse> =
            ApiRequest::get(&CcmDocApiOld::RawContent(self.doc_token).to_url());
        let response: Response<GetDocRawContentResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
