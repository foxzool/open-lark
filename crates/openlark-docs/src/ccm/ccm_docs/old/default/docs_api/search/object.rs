//! 根据搜索条件进行文档搜索。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchObjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_sort_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchObjectResponse {
    pub docs_entities: Vec<DocEntity>,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DocEntity {
    pub docs_token: String,
    pub docs_type: String,
    pub title: String,
    pub owner_id: String,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SearchObjectBuilder {
    api_req: ApiRequest<SearchObjectRequest>,
}

impl SearchObjectBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_docs_search_object".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/suite/docs-api/search/object".to_string();
        builder.api_req.body = Some(SearchObjectRequest::default());
        builder
    }

    pub fn search_key(mut self, search_key: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.search_key = Some(search_key.to_string());
        }
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.offset = Some(offset);
        }
        self
    }

    pub fn count(mut self, count: i32) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.count = Some(count);
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
