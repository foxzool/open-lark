//! 该接口用于根据 spreadsheetToken 和 protectId 查询保护范围信息。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/obtain-protection-scopes

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeRequest {
    pub protectIds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeResponse {
    pub protectedRanges: Vec<ProtectedRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRange {
    pub protectId: String,
    pub dimension: Dimension,
    pub editors: Vec<i64>,
    pub lockInfo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

impl ApiResponseTrait for BatchGetProtectedRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchGetProtectedRangeBuilder {
    api_req: ApiRequest<BatchGetProtectedRangeRequest>,
    spreadsheet_token: String,
}

impl BatchGetProtectedRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_protected_range_batch_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_get",
            builder.spreadsheet_token
        );
        builder.api_req.body = None;
        builder
    }

    pub fn protect_ids(mut self, protect_ids: Vec<String>) -> Self {
        let joined = protect_ids.join(",");
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&protectIds={}", joined));
        } else {
            self.api_req.url.push_str(&format!("?protectIds={}", joined));
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
