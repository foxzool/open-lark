//! 删除已有的条件格式，单次最多支持删除10个条件格式。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteConditionFormatRequest {
    pub sheet_condition_formats: Vec<SheetConditionFormatDelete>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormatDelete {
    pub sheet_id: String,
    pub cf_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteConditionFormatResponse {
    pub responses: Vec<ConditionFormatResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatResponse {
    pub sheet_id: String,
    pub cf_id: String,
    pub res_code: i32,
    pub res_msg: String,
}

impl ApiResponseTrait for BatchDeleteConditionFormatResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchDeleteConditionFormatBuilder {
    api_req: ApiRequest<BatchDeleteConditionFormatRequest>,
}

impl BatchDeleteConditionFormatBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_condition_format_batch_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_delete",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchDeleteConditionFormatRequest::default());
        builder
    }

    pub fn sheet_condition_formats(mut self, formats: Vec<SheetConditionFormatDelete>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.sheet_condition_formats = formats;
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
