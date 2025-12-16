//! 搜索 Wiki，用户通过关键词查询 Wiki，只能查找自己可见的 wiki。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchNodeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchNodeResponse {
    pub items: Vec<WikiNode>,
    pub page_token: String,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WikiNode {
    pub node_token: String,
    pub node_type: String,
    pub obj_token: String,
    pub obj_type: String,
    pub parent_node_token: String,
    pub node_name: String,
    pub origin_node_token: String,
    pub origin_space_id: String,
    pub has_child: bool,
    pub title: String,
}

impl ApiResponseTrait for SearchNodeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SearchNodeBuilder {
    api_req: ApiRequest<SearchNodeRequest>,
}

impl SearchNodeBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "wiki_v1_node_search".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/wiki/v1/nodes/search".to_string();
        builder.api_req.body = Some(SearchNodeRequest::default());
        builder
    }

    pub fn query(mut self, query: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.query = Some(query.to_string());
        }
        self
    }

    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.page_token = Some(page_token.to_string());
        }
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.page_size = Some(page_size);
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::User, config, option) // Typically search is user-bound
    }
}
