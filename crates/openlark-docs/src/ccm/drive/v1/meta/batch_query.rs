use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 批量获取文件元数据
///
/// 批量获取文件元数据信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/meta/batch_query
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 批量查询元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryMetaRequest {
    /// 文件token列表
    pub request_docs: Vec<RequestDoc>,
}

/// 请求文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoc {
    /// 文档token
    pub doc_token: String,
    /// 文档类型
    pub doc_type: String,
}

impl BatchQueryMetaRequest {
    pub fn new(request_docs: Vec<RequestDoc>) -> Self {
        Self { request_docs }
    }
}

/// 批量查询元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryMetaResponse {
    /// 文档元数据列表
    pub metas: Option<Vec<serde_json::Value>>,
    /// 失败列表
    pub failed_list: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchQueryMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取文件元数据
pub async fn batch_query(
    request: BatchQueryMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchQueryMetaResponse>> {
    let api_endpoint = DriveApi::BatchQueryMetas;

    let mut api_request: ApiRequest<BatchQueryMetaResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}