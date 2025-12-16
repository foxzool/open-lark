use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档富文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocContentRequest {
    /// 文档 token
    pub doc_token: String,
}

impl GetDocContentRequest {
    /// 创建新的 GetDocContentRequest
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: doc_token.into(),
        }
    }
}

/// 获取旧版文档富文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocContentResponse {
    /// 文档标题
    pub title: Option<serde_json::Value>,
    /// 文档正文
    pub body: Option<serde_json::Value>,
    /// 文档修订ID
    pub revision_id: i64,
}

impl ApiResponseTrait for GetDocContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档富文本内容
///
/// 获取结构化的文档内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
pub async fn get_doc_content(
    request: GetDocContentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetDocContentResponse>> {
    let api_endpoint = CcmDocApiOld::Content(request.doc_token.clone());
    let mut api_request: ApiRequest<GetDocContentResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc_content_request() {
        let request = GetDocContentRequest::new("doc_token");
        assert_eq!(request.doc_token, "doc_token");
    }
}
