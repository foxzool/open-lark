//! 根据 spreadsheetToken 和维度信息更新隐藏行列、单元格大小；单次操作不超过5000行或列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/update-rows-or-columns

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDimensionRangeRequest {
    pub dimension: DimensionUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionUpdate {
    pub sheetId: String,
    pub majorDimension: Option<String>,
    pub startIndex: Option<i32>,
    pub endIndex: Option<i32>,
    pub visible: Option<bool>,
    pub fixedSize: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDimensionRangeResponse {}

impl ApiResponseTrait for UpdateDimensionRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateDimensionRangeBuilder {
    api_req: ApiRequest<UpdateDimensionRangeRequest>,
}

impl UpdateDimensionRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_dimension_range_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(UpdateDimensionRangeRequest::default());
        builder
    }

    pub fn dimension(mut self, dimension: DimensionUpdate) -> Self {
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
