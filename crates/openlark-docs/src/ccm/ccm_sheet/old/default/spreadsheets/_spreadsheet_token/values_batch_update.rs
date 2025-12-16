//! 根据 spreadsheetToken 和 range 批量更新数据; 单次写入不超过5000行，100列，每个格子不超过5万字符。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/writing-multiple-ranges

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateValuesRequest {
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateValuesResponse {
    pub spreadsheetToken: String,
    pub responses: Vec<UpdateResponse>,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
}

impl ApiResponseTrait for BatchUpdateValuesResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchUpdateValuesBuilder {
    api_req: ApiRequest<BatchUpdateValuesRequest>,
}

impl BatchUpdateValuesBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_batch_update".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values_batch_update",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchUpdateValuesRequest::default());
        builder
    }

    pub fn value_ranges(mut self, value_ranges: Vec<ValueRange>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.valueRanges = value_ranges;
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
