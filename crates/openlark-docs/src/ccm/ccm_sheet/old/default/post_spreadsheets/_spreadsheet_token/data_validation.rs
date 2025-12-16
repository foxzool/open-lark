//! 根据 spreadsheetToken 、range 和下拉列表属性给单元格设置下拉列表规则；单次设置范围不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/set-dropdown

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetDataValidationRequest {
    pub range: String,
    pub dataValidationType: String,
    pub dataValidation: Option<DataValidationSetting>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationSetting {
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    pub multipleValues: bool,
    pub highlightValidData: bool,
    pub colors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetDataValidationResponse {}

impl ApiResponseTrait for SetDataValidationResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct SetDataValidationBuilder {
    api_req: ApiRequest<SetDataValidationRequest>,
}

impl SetDataValidationBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_data_validation_set".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dataValidation",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(SetDataValidationRequest::default());
        builder
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.range = range.to_string();
        }
        self
    }

    pub fn data_validation_type(mut self, data_validation_type: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.dataValidationType = data_validation_type.to_string();
        }
        self
    }

    pub fn data_validation(mut self, data_validation: DataValidationSetting) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.dataValidation = Some(data_validation);
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
