use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetFilterService,
};

/// 删除筛选
#[derive(Debug, Default)]
pub struct DeleteSheetFilterRequest {
    api_request: ApiRequest,
    spreadsheet_token: String,
    sheet_id: String,
}

impl DeleteSheetFilterRequest {
    pub fn builder() -> DeleteSheetFilterRequestBuilder {
        DeleteSheetFilterRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteSheetFilterRequestBuilder {
    request: DeleteSheetFilterRequest,
}

impl DeleteSheetFilterRequestBuilder {
    /// 表格 token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 子表 id
    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn build(self) -> DeleteSheetFilterRequest {
        self.request
    }
}

impl SpreadsheetSheetFilterService {
    /// 删除筛选
    pub async fn delete(
        &self,
        request: DeleteSheetFilterRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::DELETE;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
