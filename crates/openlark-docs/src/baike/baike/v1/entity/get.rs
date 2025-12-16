//! 获取词条详情
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};
use super::create::*;

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
        builder.api_req.req_type = "baike_entity_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.entity_id = entity_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/baike/v1/entities/{}",
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
