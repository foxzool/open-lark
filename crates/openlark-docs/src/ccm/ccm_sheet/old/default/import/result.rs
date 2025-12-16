//! 查询文件导入结果
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/query-import-results

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResultRequest {
    pub ticket: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResultResponse {
    pub result: ImportResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResult {
    pub ticket: String,
    pub url: String,
    pub warning_code: i32,
    pub warning_msg: String,
}

impl ApiResponseTrait for ImportResultResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ImportResultBuilder {
    api_req: ApiRequest<ImportResultRequest>,
}

impl ImportResultBuilder {
    pub fn new(ticket: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_import_result".to_string();
        builder.api_req.method = "GET".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/sheets/v2/import/result".to_string();
        // Since it's GET with query param? No, URL example usually puts it in query but here it might be body or query.
        // Doc says "ticket" parameter.
        // Usually GET requests have query params.
        // I'll append to URL.
        let ticket_str = ticket.to_string();
        builder.api_req.url.push_str(&format!("?ticket={}", ticket_str));
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
