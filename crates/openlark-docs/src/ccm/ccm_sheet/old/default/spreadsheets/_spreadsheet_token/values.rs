pub mod _range;
pub use _range::*;

//! 根据 spreadsheetToken 和 range 向单个范围写入数据。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateValuesRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateValuesResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for UpdateValuesResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateValuesBuilder {
    api_req: ApiRequest<UpdateValuesRequest>,
}

impl UpdateValuesBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_put".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(UpdateValuesRequest::default());
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
