//! 获取词条详情
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetEntityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetEntityResponse {
    pub entity: Entity,
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
    pub related_meta: Option<RelatedMeta>,
    pub outer_info: Option<OuterInfo>,
    pub rich_text: Option<String>,
    pub source: Option<i32>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RelatedMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OuterInfo {
    pub provider: String,
    pub outer_id: String,
}

impl ApiResponseTrait for GetEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetEntityBuilder {
    api_req: ApiRequest<GetEntityRequest>,
    entity_id: String,
}

impl GetEntityBuilder {
    pub fn new(entity_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.entity_id = entity_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/lingo/v1/entities/{}",
            builder.entity_id
        );
        builder.api_req.body = None;
        builder
    }

    pub fn provider(mut self, provider: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&provider={}", provider.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?provider={}", provider.to_string()));
        }
        self
    }

    pub fn outer_id(mut self, outer_id: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&outer_id={}", outer_id.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?outer_id={}", outer_id.to_string()));
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
