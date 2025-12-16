//! 删除免审词条
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteEntityRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteEntityResponse {}

impl ApiResponseTrait for DeleteEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DeleteEntityBuilder {
    api_req: ApiRequest<DeleteEntityRequest>,
    entity_id: String,
}

impl DeleteEntityBuilder {
    pub fn new(entity_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.entity_id = entity_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/lingo/v1/entities/{}",
            builder.entity_id
        );
        builder.api_req.body = Some(DeleteEntityRequest::default());
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
