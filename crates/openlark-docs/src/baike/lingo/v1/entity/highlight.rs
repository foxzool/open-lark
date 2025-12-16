//! 词条高亮
//!
//! doc: https://open.feishu.cn/document/lingo-v1/entity/highlight

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityResponse {
    pub phrases: Vec<Phrase>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Phrase {
    pub name: String,
    pub entity_ids: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Span {
    pub start: i32,
    pub end: i32,
}

impl ApiResponseTrait for HighlightEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct HighlightEntityBuilder {
    api_req: ApiRequest<HighlightEntityRequest>,
}

impl HighlightEntityBuilder {
    pub fn new(text: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_entity_highlight".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/lingo/v1/entities/highlight".to_string();
        builder.api_req.body = Some(HighlightEntityRequest {
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
