//! 获取旧版文档纯文本内容
//!
//! docPath: /document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

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

    pub async fn execute(self) -> SDKResult<GetDocRawContentResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;
        validate_required!(self.doc_token, "doc_token 不能为空");

        let api_request: ApiRequest<GetDocRawContentResponse> =
            ApiRequest::get(&CcmDocApiOld::RawContent(self.doc_token).to_url());
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取旧版文档纯文本内容")
    }
}
