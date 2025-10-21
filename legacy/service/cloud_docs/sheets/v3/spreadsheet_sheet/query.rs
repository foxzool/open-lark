use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::{spreadsheet_sheet::Sheet, SpreadsheetSheetService},
};

/// 获取工作表请求
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QuerySpreadsheetSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
}

impl SpreadsheetSheetService {
    /// 获取工作表
    /// 根据电子表格 token 获取表格中所有工作表及其属性信息，包括工作表
    /// ID、标题、索引位置、是否被隐藏等。
    pub async fn query(
        &self,
        request: QuerySpreadsheetSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QuerySpreadsheetSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V3_SPREADSHEET_SHEETS_QUERY.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

impl QuerySpreadsheetSheetRequest {
    pub fn builder() -> QuerySpreadsheetSheetRequestBuilder {
        QuerySpreadsheetSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct QuerySpreadsheetSheetRequestBuilder {
    request: QuerySpreadsheetSheetRequest,
}

impl QuerySpreadsheetSheetRequestBuilder {
    /// 表格的token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn build(mut self) -> QuerySpreadsheetSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct QuerySpreadsheetSheetResponse {
    pub sheets: Vec<Sheet>,
}

impl ApiResponseTrait for QuerySpreadsheetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
