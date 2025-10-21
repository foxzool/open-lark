use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::sheets::v3::{spreadsheet_sheet::Sheet, SpreadsheetSheetService},
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetSpreadsheetSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(skip)]
    sheet_id: String,
}

impl GetSpreadsheetSheetRequest {
    pub fn builder() -> GetSpreadsheetSheetRequestBuilder {
        GetSpreadsheetSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetSpreadsheetSheetRequestBuilder {
    request: GetSpreadsheetSheetRequest,
}

impl GetSpreadsheetSheetRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn build(mut self) -> GetSpreadsheetSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetService {
    /// 获取工作表
    /// 根据电子表格 token 和工作表 ID 获取工作表信息，包括工作表 ID、标题、索引位置、是否被隐藏等。
    pub async fn get(
        &self,
        request: GetSpreadsheetSheetRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<GetSpreadsheetSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_SHEET_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Deserialize, Debug)]
pub struct GetSpreadsheetSheetResponse {
    pub sheet: Sheet,
}

impl ApiResponseTrait for GetSpreadsheetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
