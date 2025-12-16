//! 根据 spreadsheetToken 和长度，在末尾增加空行/列；单次操作不超过5000行或列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/add-rows-or-columns

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddDimensionRangeRequest {
    pub dimension: Dimension,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddDimensionRangeResponse {
    pub addCount: i32,
    pub majorDimension: String,
}

impl ApiResponseTrait for AddDimensionRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct AddDimensionRangeBuilder {
    api_req: ApiRequest<AddDimensionRangeRequest>,
}

impl AddDimensionRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_dimension_range_add".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(AddDimensionRangeRequest::default());
        builder
    }

    pub fn dimension(mut self, dimension: Dimension) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.dimension = dimension;
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
