//! 根据 spreadsheetToken 和 range 遇到空行则进行覆盖追加或新增行追加数据。 空行：默认该行第一个格子是空，则认为是空行；单次写入不超过5000行，100列，每个格子不超过5万字符。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/append-data

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesAppendRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesAppendResponse {
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

impl ApiResponseTrait for ValuesAppendResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ValuesAppendBuilder {
    api_req: ApiRequest<ValuesAppendRequest>,
}

impl ValuesAppendBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_append".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values_append",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(ValuesAppendRequest::default());
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
