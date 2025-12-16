//! 提取潜在的词条
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityResponse {
    pub entity_word: String,
    pub alias: Option<Vec<String>>,
}

impl ApiResponseTrait for ExtractEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ExtractEntityBuilder {
    api_req: ApiRequest<ExtractEntityRequest>,
}

impl ExtractEntityBuilder {
    pub fn new(text: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "baike_entity_extract".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/baike/v1/entities/extract".to_string();
        builder.api_req.body = Some(ExtractEntityRequest {
            text: text.to_string(),
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
