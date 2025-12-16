/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectRequest {
    pub search_key: Option<String>,
    pub count: Option<i32>,
    pub offset: Option<i32>,
    pub owner_ids: Option<Vec<String>>,
    pub chat_ids: Option<Vec<String>>,
    pub docs_types: Option<Vec<String>>,
    pub docs_sort_type: Option<String>,
    pub is_folder: Option<bool>,
}

impl SearchObjectRequest {
    pub fn new() -> Self {
        Self {
            search_key: None,
            count: None,
            offset: None,
            owner_ids: None,
            chat_ids: None,
            docs_types: None,
            docs_sort_type: None,
            is_folder: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    pub docs_entities: Option<Vec<DocsEntity>>,
    pub total: Option<i32>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsEntity {
    pub docs_token: String,
    pub docs_type: String,
    pub title: String,
    pub owner_id: String,
    pub create_time: i64,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn search_object(
    request: SearchObjectRequest,
    config: &Config,
) -> SDKResult<Response<SearchObjectResponse>> {
    let mut api_request: ApiRequest<SearchObjectResponse> = ApiRequest::post("/open-apis/suite/docs-api/search/object")
        .body(serde_json::to_value(request)?);
    
    Transport::request(api_request, config, None).await
}
