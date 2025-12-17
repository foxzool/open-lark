//! 模糊搜索词条
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/search

use super::create::*;
use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityResponse {
    pub entities: Vec<Entity>,
}

impl ApiResponseTrait for SearchEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SearchEntityBuilder {
    api_req: ApiRequest<SearchEntityRequest>,
}

impl SearchEntityBuilder {
    pub fn new(query: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.method = openlark_core::api::HttpMethod::Post;
        builder.api_req.url =
            "https://open.feishu.cn/open-apis/baike/v1/entities/search".to_string();
        builder.api_req.body = Some(SearchEntityRequest {
            query: query.to_string(),
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
