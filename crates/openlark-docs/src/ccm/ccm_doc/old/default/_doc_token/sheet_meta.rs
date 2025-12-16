//! 根据 docToken 获取文档中的电子表格的元数据。
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaResponse {
    pub sheetId: i32,
    pub title: String,
    pub rowCount: i32,
    pub colCount: i32,
}

impl ApiResponseTrait for GetDocSheetMetaResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetDocSheetMetaBuilder {
    api_req: ApiRequest<GetDocSheetMetaRequest>,
    doc_token: String,
}

impl GetDocSheetMetaBuilder {
    pub fn new(doc_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_doc_sheet_meta_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.doc_token = doc_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/doc/v2/{}/sheet_meta",
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
