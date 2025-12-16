use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocsApiOld;

/// 获取元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaRequest {
    /// token列表
    pub request_docs: Vec<RequestDoc>,
    /// 是否获取更多信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_more_info: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoc {
    /// 文档 token
    pub docs_token: String,
    /// 文档类型
    pub docs_type: String,
}

impl GetMetaRequest {
    /// 创建新的 GetMetaRequest
    pub fn new(request_docs: Vec<RequestDoc>) -> Self {
        Self {
            request_docs,
            with_more_info: None,
        }
    }
}

/// 获取元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaResponse {
    /// 元数据列表
    pub docs_metas: Vec<serde_json::Value>,
}

impl ApiResponseTrait for GetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取元数据
///
/// 根据 token 获取各类文件的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
pub async fn meta(
    request: GetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetMetaResponse>> {
    let api_endpoint = CcmDocsApiOld::Meta;
    let mut api_request: ApiRequest<GetMetaResponse> = ApiRequest::post(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_meta_request() {
        let request = GetMetaRequest::new(vec![]);
        assert!(request.request_docs.is_empty());
    }
}
