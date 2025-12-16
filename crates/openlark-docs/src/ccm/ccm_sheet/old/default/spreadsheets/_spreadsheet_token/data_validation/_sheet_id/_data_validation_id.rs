//! 该接口用于根据 sheetId 和 dataValidationId 查询单个下拉列表的详细信息。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-single-datavalidation

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDataValidationRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDataValidationResponse {
    pub spreadsheetToken: String,
    pub sheetId: String,
    pub dataValidation: DataValidationDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationDetail {
    pub dataValidationId: i32,
    pub dataValidationType: String,
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    pub multipleValues: bool,
    pub highlightValidData: bool,
    pub colors: Vec<String>,
}

impl ApiResponseTrait for GetDataValidationResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetDataValidationBuilder {
    api_req: ApiRequest<GetDataValidationRequest>,
    spreadsheet_token: String,
    sheet_id: String,
    data_validation_id: String,
}

impl GetDataValidationBuilder {
    pub fn new(spreadsheet_token: impl ToString, sheet_id: impl ToString, data_validation_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_data_validation_get_single".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.sheet_id = sheet_id.to_string();
        builder.data_validation_id = data_validation_id.to_string();
        builder.api_req.body = None;
        builder
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dataValidation/{}/{}",
            self.spreadsheet_token,
            self.sheet_id,
            self.data_validation_id
        );
        req.build(AccessTokenType::Tenant, config, option)
    }
}
