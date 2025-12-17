//! 更新免审词条
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/update

use super::create::*;
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
pub struct UpdateEntityResponse {
    pub entity: Entity,
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
        builder.api_req.method = openlark_core::api::HttpMethod::Put;
        builder.entity_id = entity_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/baike/v1/entities/{}",
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
