//! 根据 token 获取各类文件的元数据。
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocsMetaRequest {
    pub request_docs: Vec<RequestDoc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestDoc {
    pub docs_token: String,
    pub docs_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocsMetaResponse {
    pub docs_metas: Vec<DocMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DocMeta {
    pub docs_token: String,
    pub docs_type: String,
    pub title: String,
    pub owner_id: String,
    pub create_time: i64,
    pub latest_modify_user_id: String,
    pub latest_modify_time: i64,
}

impl ApiResponseTrait for GetDocsMetaResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetDocsMetaBuilder {
    api_req: ApiRequest<GetDocsMetaRequest>,
}

impl GetDocsMetaBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_docs_meta_get".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/suite/docs-api/meta".to_string();
        builder.api_req.body = Some(GetDocsMetaRequest::default());
        builder
    }

    pub fn request_docs(mut self, request_docs: Vec<RequestDoc>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.request_docs = request_docs;
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
