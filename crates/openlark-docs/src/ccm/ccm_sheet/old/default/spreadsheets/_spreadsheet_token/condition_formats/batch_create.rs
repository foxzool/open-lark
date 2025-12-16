//! 该接口用于根据 spreadsheetToken 和 sheetId 批量添加条件格式。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/condition-format-create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateConditionFormatRequest {
    pub sheet_condition_formats: Vec<SheetConditionFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormat {
    pub sheet_id: String,
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub ranges: Vec<String>,
    pub rule_type: String,
    pub attrs: Vec<ConditionFormatAttr>,
    pub style: Option<ConditionFormatStyle>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatAttr {
    pub operator: String,
    pub formula: Option<Vec<String>>,
    pub text: Option<String>,
    pub time_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatStyle {
    pub font: Option<Font>,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Font {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateConditionFormatResponse {
    pub responses: Vec<ConditionFormatResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatResponse {
    pub sheet_id: String,
    pub cf_id: String,
    pub res_code: i32,
    pub res_msg: String,
}

impl ApiResponseTrait for BatchCreateConditionFormatResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchCreateConditionFormatBuilder {
    api_req: ApiRequest<BatchCreateConditionFormatRequest>,
}

impl BatchCreateConditionFormatBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_condition_format_batch_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_create",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(BatchCreateConditionFormatRequest::default());
        builder
    }

    pub fn sheet_condition_formats(mut self, formats: Vec<SheetConditionFormat>) -> Self {
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
