//! 该接口用于根据 spreadsheetToken 和维度信息删除行/列 。单次删除最大5000行/列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/-delete-rows-or-columns

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDimensionRangeRequest {
    pub dimension: DimensionDelete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionDelete {
    pub sheetId: String,
    pub majorDimension: Option<String>,
    pub startIndex: Option<i32>,
    pub endIndex: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDimensionRangeResponse {
    pub delCount: i32,
    pub majorDimension: String,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DeleteDimensionRangeBuilder {
    api_req: ApiRequest<DeleteDimensionRangeRequest>,
}

impl DeleteDimensionRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_dimension_range_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(DeleteDimensionRangeRequest::default());
        builder
    }

    pub fn dimension(mut self, dimension: DimensionDelete) -> Self {
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
