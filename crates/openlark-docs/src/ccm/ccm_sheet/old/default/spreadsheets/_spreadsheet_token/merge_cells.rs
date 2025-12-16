//! 根据 spreadsheetToken 和维度信息合并单元格；单次操作不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeCellsRequest {
    pub ranges: Vec<String>,
    pub mergeType: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeCellsResponse {
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct MergeCellsBuilder {
    api_req: ApiRequest<MergeCellsRequest>,
}

impl MergeCellsBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_merge_cells".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/merge_cells",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(MergeCellsRequest::default());
        builder
    }

    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.ranges = ranges;
        }
        self
    }

    pub fn merge_type(mut self, merge_type: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.mergeType = merge_type.to_string();
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
