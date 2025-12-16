//! 更新免审词条
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateEntityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<Term>>,
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
pub struct UpdateEntityResponse {
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

impl ApiResponseTrait for UpdateEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateEntityBuilder {
    api_req: ApiRequest<UpdateEntityRequest>,
    entity_id: String,
}

impl UpdateEntityBuilder {
    pub fn new(entity_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.entity_id = entity_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/lingo/v1/entities/{}",
            builder.entity_id
        );
        builder.api_req.body = Some(UpdateEntityRequest::default());
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
