//! 模糊搜索词条
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/search

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_filter: Option<ClassificationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassificationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityResponse {
    pub entities: Vec<Entity>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    pub id: String,
    pub main_keys: Vec<Term>,
    pub aliases: Option<Vec<Term>>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Term {
    pub key: String,
}

impl ApiResponseTrait for SearchEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SearchEntityBuilder {
    api_req: ApiRequest<SearchEntityRequest>,
}

impl SearchEntityBuilder {
    pub fn new(query: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_search".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/lingo/v1/entities/search".to_string();
        builder.api_req.body = Some(SearchEntityRequest {
            query: query.to_string(),
            ..Default::default()
        });
        builder
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
