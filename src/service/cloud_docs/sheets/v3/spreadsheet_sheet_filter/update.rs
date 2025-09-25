use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::{
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService,
    },
};

/// 更新子表筛选范围中的列筛选条件
#[derive(Debug, Serialize, Default)]
pub struct UpdateSheetFilterRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(skip)]
    sheet_id: String,
    /// 设置筛选条件的列
    col: String,
    /// 筛选的条件
    condition: SheetFilterCondition,
}

impl UpdateSheetFilterRequest {
    pub fn builder() -> UpdateSheetFilterRequestBuilder {
        UpdateSheetFilterRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateSheetFilterRequestBuilder {
    request: UpdateSheetFilterRequest,
}

impl UpdateSheetFilterRequestBuilder {
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

    /// 设置筛选条件的列
    pub fn col(mut self, col: impl ToString) -> Self {
        self.request.col = col.to_string();
        self
    }

    /// 筛选的条件
    pub fn condition(mut self, condition: SheetFilterCondition) -> Self {
        self.request.condition = condition;
        self
    }

    pub fn build(mut self) -> UpdateSheetFilterRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetFilterService {
    /// 更新筛选
    pub async fn update(
        &self,
        request: UpdateSheetFilterRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
