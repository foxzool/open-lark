//! 该接口用于根据 spreadsheetToken 和 protectId 删除保护范围。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/delete-protection-scopes

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDelProtectedRangeRequest {
    pub protectIds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDelProtectedRangeResponse {
    pub delProtectIds: Vec<String>,
}

impl ApiResponseTrait for BatchDelProtectedRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchDelProtectedRangeBuilder {
    api_req: ApiRequest<BatchDelProtectedRangeRequest>,
}

impl BatchDelProtectedRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_protected_range_batch_del".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_del",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchDelProtectedRangeRequest::default());
        builder
    }

    pub fn protect_ids(mut self, protect_ids: Vec<String>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.protectIds = protect_ids;
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
