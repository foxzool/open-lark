/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: /document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectReq {
    pub search_key: Option<String>,
    pub count: Option<i32>,
    pub offset: Option<i32>,
    pub owner_ids: Option<Vec<String>>,
    pub chat_ids: Option<Vec<String>>,
    pub docs_types: Option<Vec<String>>,
    pub docs_sort_type: Option<String>,
    pub is_folder: Option<bool>,
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

/// 搜索云文档请求（旧版）
pub struct SearchObjectRequest {
    config: Config,
    req: SearchObjectReq,
}

impl SearchObjectRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: SearchObjectReq {
                search_key: None,
                count: None,
                offset: None,
                owner_ids: None,
                chat_ids: None,
                docs_types: None,
                docs_sort_type: None,
                is_folder: None,
            },
        }
    }

    pub fn search_key(mut self, search_key: impl Into<String>) -> Self {
        self.req.search_key = Some(search_key.into());
        self
    }

    pub fn count(mut self, count: i32) -> Self {
        self.req.count = Some(count);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.req.offset = Some(offset);
        self
    }

    pub fn owner_ids(mut self, owner_ids: Vec<String>) -> Self {
        self.req.owner_ids = Some(owner_ids);
        self
    }

    pub fn chat_ids(mut self, chat_ids: Vec<String>) -> Self {
        self.req.chat_ids = Some(chat_ids);
        self
    }

    pub fn docs_types(mut self, docs_types: Vec<String>) -> Self {
        self.req.docs_types = Some(docs_types);
        self
    }

    pub fn docs_sort_type(mut self, docs_sort_type: impl Into<String>) -> Self {
        self.req.docs_sort_type = Some(docs_sort_type.into());
        self
    }

    pub fn is_folder(mut self, is_folder: bool) -> Self {
        self.req.is_folder = Some(is_folder);
        self
    }

    pub async fn send(self) -> SDKResult<SearchObjectResponse> {
        use crate::common::api_endpoints::CcmDocsApiOld;

        let api_request: ApiRequest<SearchObjectResponse> =
            ApiRequest::post(&CcmDocsApiOld::SearchObject.to_url())
                .body(serialize_params(&self.req, "搜索云文档")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "搜索云文档")
    }
}
