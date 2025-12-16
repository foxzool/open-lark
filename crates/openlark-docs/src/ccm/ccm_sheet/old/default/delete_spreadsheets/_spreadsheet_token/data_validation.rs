//! 根据 spreadsheetToken 和 range 删除range内的下拉列表设置。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/delete-datavalidation

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDataValidationRequest {
    pub range: String,
    pub dataValidationIds: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDataValidationResponse {
    pub range: String,
}

impl ApiResponseTrait for DeleteDataValidationResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DeleteDataValidationBuilder {
    api_req: ApiRequest<DeleteDataValidationRequest>,
}

impl DeleteDataValidationBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_data_validation_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/dataValidation",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(DeleteDataValidationRequest::default());
        builder
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.range = range.to_string();
        }
        self
    }

    pub fn data_validation_ids(mut self, iso: Vec<i32>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.dataValidationIds = Some(iso);
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
