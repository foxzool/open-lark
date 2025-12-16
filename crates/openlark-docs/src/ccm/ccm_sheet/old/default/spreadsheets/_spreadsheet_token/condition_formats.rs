//! 根据sheetId查询详细的条件格式信息，最多支持同时查询10个sheetId。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-get

pub mod batch_create;
pub mod batch_update;
pub mod batch_delete;
pub use batch_create::*;
pub use batch_update::*;
pub use batch_delete::*;

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_ids: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatResponse {
    pub sheet_condition_formats: Vec<SheetConditionFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormat {
    pub sheet_id: String,
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub cf_id: String,
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

impl ApiResponseTrait for GetConditionFormatResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetConditionFormatBuilder {
    api_req: ApiRequest<GetConditionFormatRequest>,
    spreadsheet_token: String,
}

impl GetConditionFormatBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_condition_format_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/condition_formats",
            builder.spreadsheet_token
        );
        builder.api_req.body = None;
        builder
    }

    pub fn sheet_ids(mut self, sheet_ids: Vec<String>) -> Self {
        let joined = sheet_ids.join(",");
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&sheet_ids={}", joined));
        } else {
            self.api_req.url.push_str(&format!("?sheet_ids={}", joined));
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
