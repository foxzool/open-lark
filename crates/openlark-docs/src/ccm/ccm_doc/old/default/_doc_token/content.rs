//! 获取旧版文档富文本内容
//!
//! docPath: /document/ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

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
        validate_required!(self.doc_token, "doc_token 不能为空");

        let api_request: ApiRequest<GetDocContentResponse> =
            ApiRequest::get(&CcmDocApiOld::Content(self.doc_token).to_url());
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取旧版文档富文本内容")
    }
}
