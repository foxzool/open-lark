//! 根据 spreadsheetToken 和维度信息增加多个保护范围；单次操作不超过5000行或列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/add-locked-cells

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddProtectedDimensionRequest {
    pub addProtectedDimension: Vec<ProtectedDimension>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimension {
    pub dimension: Dimension,
    pub editors: Option<Vec<i64>>, // User IDs
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddProtectedDimensionResponse {
    pub addProtectedDimension: Vec<ProtectedDimensionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimensionResult {
    pub dimension: Dimension,
    pub editors: Vec<i64>,
    pub protectId: String,
    pub lockInfo: String,
}

impl ApiResponseTrait for AddProtectedDimensionResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct AddProtectedDimensionBuilder {
    api_req: ApiRequest<AddProtectedDimensionRequest>,
}

impl AddProtectedDimensionBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_protected_dimension_add".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/protected_dimension",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(AddProtectedDimensionRequest::default());
        builder
    }

    pub fn dimensions(mut self, dimensions: Vec<ProtectedDimension>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.addProtectedDimension = dimensions;
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
