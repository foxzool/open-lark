use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档纯文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocRawContentRequest {
    /// 文档 token
    pub doc_token: String,
}

impl GetDocRawContentRequest {
    /// 创建新的 GetDocRawContentRequest
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: doc_token.into(),
        }
    }
}

/// 获取旧版文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocRawContentResponse {
    /// 文档纯文本内容
    pub content: String,
}

impl ApiResponseTrait for GetDocRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
pub async fn get_doc_raw_content(
    request: GetDocRawContentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetDocRawContentResponse>> {
    let api_endpoint = CcmDocApiOld::RawContent(request.doc_token.clone());
    let mut api_request: ApiRequest<GetDocRawContentResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc_raw_content_request() {
        let request = GetDocRawContentRequest::new("doc_token");
        assert_eq!(request.doc_token, "doc_token");
    }
}
