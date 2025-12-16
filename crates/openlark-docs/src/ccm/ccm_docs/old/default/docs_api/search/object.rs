use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocsApiOld;

/// 搜索云文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectRequest {
    /// 关键词
    pub search_key: Option<String>,
    /// 数量
    pub count: Option<i32>,
    /// 偏移
    pub offset: Option<i32>,
    /// 所有者IDs
    pub owner_ids: Option<Vec<String>>,
    /// 聊天IDs
    pub chat_ids: Option<Vec<String>>,
    /// 文档类型
    pub docs_types: Option<Vec<String>>,
    /// 文档创建时间范围
    pub docs_create_time_start: Option<i64>,
    pub docs_create_time_end: Option<i64>,
    /// 文档打开时间范围
    pub docs_open_time_start: Option<i64>,
    pub docs_open_time_end: Option<i64>,
}

impl SearchObjectRequest {
    /// 创建新的 SearchObjectRequest
    pub fn new() -> Self {
        Self {
            search_key: None,
            count: None,
            offset: None,
            owner_ids: None,
            chat_ids: None,
            docs_types: None,
            docs_create_time_start: None,
            docs_create_time_end: None,
            docs_open_time_start: None,
            docs_open_time_end: None,
        }
    }
}

/// 搜索云文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    /// 文档列表
    pub docs_entities: Vec<serde_json::Value>,
    /// 总数
    pub total: i32,
    /// 是否有更多
    pub has_more: bool,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
pub async fn object(
    request: SearchObjectRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SearchObjectResponse>> {
    let api_endpoint = CcmDocsApiOld::SearchObject;
    let mut api_request: ApiRequest<SearchObjectResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_search_object_request() {
        let _request = SearchObjectRequest::new();
    }
}
