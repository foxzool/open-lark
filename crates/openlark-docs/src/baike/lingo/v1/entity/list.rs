//! 获取词条列表
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListEntityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListEntityResponse {
    pub items: Vec<Entity>,
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    pub id: String,
    pub main_keys: Vec<Term>,
    pub aliases: Option<Vec<Term>>,
    pub description: String,
    pub creator: String,
    pub create_time: i64,
    pub updater: String,
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Term {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DisplayStatus {
    pub allow_highlight: bool,
    pub allow_search: bool,
}

impl ApiResponseTrait for ListEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ListEntityBuilder {
    api_req: ApiRequest<ListEntityRequest>,
}

impl ListEntityBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_list".to_string();
        builder.api_req.method = "GET".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/lingo/v1/entities".to_string();
        builder.api_req.body = None;
        builder
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&page_size={}", page_size));
        } else {
            self.api_req.url.push_str(&format!("?page_size={}", page_size));
        }
        self
    }

    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&page_token={}", page_token.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?page_token={}", page_token.to_string()));
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
