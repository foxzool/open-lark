//! 根据 spreadsheetToken 和 range 读取表格数据，返回数据；单次读取不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetValuesRangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetValuesRangeResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for GetValuesRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetValuesRangeBuilder {
    api_req: ApiRequest<GetValuesRangeRequest>,
    spreadsheet_token: String,
    range: String,
}

impl GetValuesRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString, range: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_range_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.range = range.to_string();
        builder.api_req.body = None; // GET has no body usually, depends on client impl
        builder
    }

    pub fn value_render_option(mut self, option: impl ToString) -> Self {
        if self.api_req.body.is_none() {
            self.api_req.body = Some(GetValuesRangeRequest::default());
        }
        if let Some(body) = &mut self.api_req.body {
            body.valueRenderOption = Some(option.to_string());
        }
        self
    }

    pub fn date_time_render_option(mut self, option: impl ToString) -> Self {
        if self.api_req.body.is_none() {
            self.api_req.body = Some(GetValuesRangeRequest::default());
        }
        if let Some(body) = &mut self.api_req.body {
            body.dateTimeRenderOption = Some(option.to_string());
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values/{}",
            self.spreadsheet_token,
            self.range
        );
        req.build(AccessTokenType::Tenant, config, option)
    }
}
