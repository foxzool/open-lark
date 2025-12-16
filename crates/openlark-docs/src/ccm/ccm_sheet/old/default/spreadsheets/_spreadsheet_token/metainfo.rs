//! 获取表格元数据
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/obtain-spreadsheet-metadata

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaResponse {
    pub spreadsheet_token: String,
    pub properties: SpreadsheetProperties,
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    pub title: String,
    pub owner_user: Option<i64>,
    pub sheet_count: i32,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sheet {
    pub sheet_id: String,
    pub title: String,
    pub index: i32,
    pub row_count: i32,
    pub column_count: i32,
    pub frozen_row_count: i32,
    pub frozen_col_count: i32,
}

impl ApiResponseTrait for GetSpreadsheetMetaResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetSpreadsheetMetaBuilder {
    api_req: ApiRequest<GetSpreadsheetMetaRequest>,
    spreadsheet_token: String,
}

impl GetSpreadsheetMetaBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_spreadsheet_meta_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.spreadsheet_token = spreadsheet_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/metainfo",
            builder.spreadsheet_token
        );
        builder.api_req.body = None;
        builder
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
