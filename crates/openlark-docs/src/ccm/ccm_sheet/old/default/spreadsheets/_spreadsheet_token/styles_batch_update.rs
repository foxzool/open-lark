//! 根据 spreadsheetToken 、range和样式信息 批量更新单元格样式；单次写入不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchSetStyleRequest {
    pub data: Vec<BatchStyleData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchStyleData {
    pub ranges: Vec<String>,
    pub style: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchSetStyleResponse {
    pub spreadsheetToken: String,
    pub totalUpdatedRows: i32,
    pub totalUpdatedColumns: i32,
    pub totalUpdatedCells: i32,
}

impl ApiResponseTrait for BatchSetStyleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchSetStyleBuilder {
    api_req: ApiRequest<BatchSetStyleRequest>,
}

impl BatchSetStyleBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_style_batch_set".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/styles_batch_update",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchSetStyleRequest::default());
        builder
    }

    pub fn data(mut self, data: Vec<BatchStyleData>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.data = data;
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
