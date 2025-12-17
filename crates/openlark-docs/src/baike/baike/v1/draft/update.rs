//! 更新草稿
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/draft/update

use super::super::entity::*;
use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDraftRequest {
    pub entity: Entity,
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
        builder.api_req.method = openlark_core::api::HttpMethod::Put;
        builder.draft_id = draft_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/baike/v1/drafts/{}",
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
