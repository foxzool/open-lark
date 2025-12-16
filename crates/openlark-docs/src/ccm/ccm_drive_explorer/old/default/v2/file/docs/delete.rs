use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 删除Doc请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocRequest {
    /// 文档 token
    #[serde(skip)]
    pub doc_token: String,
}

impl DeleteDocRequest {
    /// 创建新的 DeleteDocRequest
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: doc_token.into(),
        }
    }
}

/// 删除Doc响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocResponse {
    /// id
    pub id: String,
    /// result
    pub result: bool,
}

impl ApiResponseTrait for DeleteDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除Doc
///
/// 根据 docToken 删除对应的 Docs 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc
pub async fn delete(
    request: DeleteDocRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteDocResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::FileDocs(request.doc_token.clone());
    let mut api_request: ApiRequest<DeleteDocResponse> = ApiRequest::delete(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_doc_request() {
        let request = DeleteDocRequest::new("doc_token");
        assert_eq!(request.doc_token, "doc_token");
    }
}
