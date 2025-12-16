//! 更新草稿
//!
//! doc: https://open.feishu.cn/document/lingo-v1/draft/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDraftRequest {
    pub entity: Entity,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    pub main_keys: Vec<Term>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
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
pub struct UpdateDraftResponse {
    pub draft_id: String,
    pub entity: Entity,
}

impl ApiResponseTrait for UpdateDraftResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateDraftBuilder {
    api_req: ApiRequest<UpdateDraftRequest>,
    draft_id: String,
}

impl UpdateDraftBuilder {
    pub fn new(draft_id: impl ToString, entity: Entity) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_draft_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.draft_id = draft_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/lingo/v1/drafts/{}",
            builder.draft_id
        );
        builder.api_req.body = Some(UpdateDraftRequest { entity });
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
