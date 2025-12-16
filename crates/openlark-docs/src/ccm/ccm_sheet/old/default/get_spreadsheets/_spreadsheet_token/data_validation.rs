//! 根据 spreadsheetToken 、range 查询range内的下拉列表设置信息；单次查询范围不超过5000行，100列。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-datavalidation

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryDataValidationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataValidationType: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryDataValidationResponse {
    pub spreadsheetToken: String,
    pub dataValidations: Vec<DataValidation>,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidation {
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

impl ApiResponseTrait for QueryDataValidationResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct QueryDataValidationBuilder {
    api_req: ApiRequest<QueryDataValidationRequest>,
    spreadsheet_token: String,
}

impl QueryDataValidationBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_data_validation_query".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dataValidation",
            builder.spreadsheet_token
        );
        builder.api_req.body = None;
        builder
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&range={}", range.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?range={}", range.to_string()));
        }
        self
    }

    pub fn data_validation_type(mut self, data_validation_type: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&dataValidationType={}", data_validation_type.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?dataValidationType={}", data_validation_type.to_string()));
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
