//! 获取旧版文档富文本内容
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocContentReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocContentResponse {
    pub content: String,
    pub revision: i32,
}

impl ApiResponseTrait for GetDocContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档富文本内容请求
pub struct GetDocContentRequest {
    config: Config,
    doc_token: String,
}

impl GetDocContentRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<GetDocContentResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<GetDocContentResponse> =
            ApiRequest::get(&CcmDocApiOld::Content(self.doc_token).to_url());
        let response: Response<GetDocContentResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
