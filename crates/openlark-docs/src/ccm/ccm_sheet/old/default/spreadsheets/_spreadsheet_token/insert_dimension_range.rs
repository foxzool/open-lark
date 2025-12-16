//! 根据 spreadsheetToken 和维度信息 插入空行/列。如 startIndex=3， endIndex=7，则从第 4 行开始开始插入行列，一直到第 7 行，共插入 4 行；单次操作不超过5000行或列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/insert-rows-or-columns

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeRequest {
    pub dimension: DimensionInsert,
    pub inheritStyle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionInsert {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeResponse {}

impl ApiResponseTrait for InsertDimensionRangeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct InsertDimensionRangeBuilder {
    api_req: ApiRequest<InsertDimensionRangeRequest>,
}

impl InsertDimensionRangeBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_dimension_range_insert".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/insert_dimension_range",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(InsertDimensionRangeRequest::default());
        builder
    }

    pub fn dimension(mut self, dimension: DimensionInsert) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.dimension = dimension;
        }
        self
    }

    pub fn inherit_style(mut self, inherit_style: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.inheritStyle = Some(inherit_style.to_string());
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
