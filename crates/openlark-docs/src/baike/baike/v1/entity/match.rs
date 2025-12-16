//! 精准搜索词条
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/match

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MatchEntityRequest {
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MatchEntityResponse {
    pub results: Vec<MatchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MatchResult {
    pub entity_id: String,
    #[serde(rename = "type")]
    pub type_: i32,
}

impl ApiResponseTrait for MatchEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct MatchEntityBuilder {
    api_req: ApiRequest<MatchEntityRequest>,
}

impl MatchEntityBuilder {
    pub fn new(word: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "baike_entity_match".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/baike/v1/entities/match".to_string();
        builder.api_req.body = Some(MatchEntityRequest {
            word: word.to_string(),
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
