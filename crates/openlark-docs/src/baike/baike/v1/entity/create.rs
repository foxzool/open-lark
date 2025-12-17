//! 创建免审词条
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEntityRequest {
    pub main_keys: Vec<Term>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEntityResponse {
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

impl ApiResponseTrait for CreateEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateEntityBuilder {
    api_req: ApiRequest<CreateEntityRequest>,
}

impl CreateEntityBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.method = openlark_core::api::HttpMethod::Post;
        builder.api_req.url = "https://open.feishu.cn/open-apis/baike/v1/entities".to_string();
        builder.api_req.body = Some(CreateEntityRequest::default());
        builder
    }

    pub fn main_keys(mut self, main_keys: Vec<Term>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.main_keys = main_keys;
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
