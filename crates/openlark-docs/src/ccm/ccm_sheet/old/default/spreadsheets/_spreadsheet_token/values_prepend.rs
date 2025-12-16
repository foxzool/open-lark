//! 根据 spreadsheetToken 和 range 向范围之前增加相应数据的行和相应的数据，相当于数组的插入操作；单次写入不超过5000行，100列，每个格子不超过5万字符。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/prepend-data

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesPrependRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesPrependResponse {
    pub spreadsheetToken: String,
    pub tableRange: String,
    pub revision: i32,
    pub updates: UpdateResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResult {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for ValuesPrependResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ValuesPrependBuilder {
    api_req: ApiRequest<ValuesPrependRequest>,
}

impl ValuesPrependBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_prepend".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values_prepend",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(ValuesPrependRequest::default());
        builder
    }

    pub fn value_range(mut self, value_range: ValueRange) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.valueRange = value_range;
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
