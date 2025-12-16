//! 更新表格属性
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/update-spreadsheet-properties

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpreadsheetPropertiesRequest {
    pub properties: SpreadsheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpreadsheetPropertiesResponse {
    pub spreadsheet_token: String,
    pub title: String,
}

impl ApiResponseTrait for UpdateSpreadsheetPropertiesResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateSpreadsheetPropertiesBuilder {
    api_req: ApiRequest<UpdateSpreadsheetPropertiesRequest>,
    spreadsheet_token: String,
}

impl UpdateSpreadsheetPropertiesBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_spreadsheet_properties_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/properties",
            builder.spreadsheet_token
        );
        builder.api_req.body = Some(UpdateSpreadsheetPropertiesRequest::default());
        builder
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.properties.title = Some(title.to_string());
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
