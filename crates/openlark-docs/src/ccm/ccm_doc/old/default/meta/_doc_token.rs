//! 根据 docToken 获取元数据。
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaResponse {
    pub create_date: String,
    pub create_time: i64,
    pub creator: String,
    pub create_user_name: String,
    pub delete_flag: i32,
    pub edit_time: i64,
    pub edit_user_name: String,
    pub owner_user_name: String,
    pub server_time: i64,
    pub tenant_id: String,
    pub title: String,
    pub type_: i32,
    pub url: String,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetDocMetaBuilder {
    api_req: ApiRequest<GetDocMetaRequest>,
    doc_token: String,
}

impl GetDocMetaBuilder {
    pub fn new(doc_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_doc_meta_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.doc_token = doc_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/doc/v2/meta/{}",
            builder.doc_token
        );
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
