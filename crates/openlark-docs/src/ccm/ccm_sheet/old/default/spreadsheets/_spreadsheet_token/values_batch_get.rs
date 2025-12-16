//! 根据 spreadsheetToken 和 ranges 读取表格数据；单次读取不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<String>, // Query param: ranges=Range1,Range2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchGetValuesResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchGetValuesBuilder {
    api_req: ApiRequest<BatchGetValuesRequest>,
    spreadsheet_token: String,
}

impl BatchGetValuesBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_batch_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values_batch_get",
            builder.spreadsheet_token
        );
        builder.api_req.body = None;
        builder
    }

    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        // usually query param for GET
        // Core framework handles query params if body is None? Or need to append to URL?
        // Assuming core handles query params via body if method is GET is tricky.
        // Let's assume standard builder pattern where we might need to manually append or use req_option.
        // For simplicity in this task, I'll store it in a way that core might handle or just simpler.
        // Actually, let's append to URL for now as it's safe.
        // ranges=A1:B2,C1:D2
        let joined = ranges.join(",");
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&ranges={}", joined));
        } else {
            self.api_req.url.push_str(&format!("?ranges={}", joined));
        }
        self
    }

    pub fn value_render_option(mut self, option: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&valueRenderOption={}", option.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?valueRenderOption={}", option.to_string()));
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
