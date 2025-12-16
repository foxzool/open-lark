//! 根据 spreadsheetToken 和维度信息拆分单元格；单次操作不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/split-cells

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UnmergeCellsRequest {
    pub ranges: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UnmergeCellsResponse {
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for UnmergeCellsResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UnmergeCellsBuilder {
    api_req: ApiRequest<UnmergeCellsRequest>,
}

impl UnmergeCellsBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_unmerge_cells".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/unmerge_cells",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(UnmergeCellsRequest::default());
        builder
    }

    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.ranges = ranges;
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
