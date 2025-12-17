//! 创建草稿
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/draft/create

use super::super::entity::*;
use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDraftRequest {
    pub entity: Entity,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDraftResponse {
    pub draft_id: String,
    pub entity: Entity,
}

impl ApiResponseTrait for CreateDraftResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateDraftBuilder {
    api_req: ApiRequest<CreateDraftRequest>,
}

impl CreateDraftBuilder {
    pub fn new(entity: Entity) -> Self {
        let mut builder = Self::default();
        builder.api_req.method = openlark_core::api::HttpMethod::Post;
        builder.api_req.url = "https://open.feishu.cn/open-apis/baike/v1/drafts".to_string();
        builder.api_req.body = Some(CreateDraftRequest { entity });
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
