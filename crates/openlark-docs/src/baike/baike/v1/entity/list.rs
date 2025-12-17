//! 获取词条列表
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/entity/list

use super::create::*;
use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListEntityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListEntityResponse {
    pub items: Vec<Entity>,
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

impl ApiResponseTrait for ListEntityResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ListEntityBuilder {
    api_req: ApiRequest<ListEntityRequest>,
}

impl ListEntityBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.method = openlark_core::api::HttpMethod::Get;
        builder.api_req.url = "https://open.feishu.cn/open-apis/baike/v1/entities".to_string();
        builder.api_req.body = None;
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
