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
    impl_executable_builder_owned,
    service::sheets::v3::{
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService,
    },
};

/// 在子表内创建筛选
#[derive(Debug, Serialize, Default)]
pub struct CreateSheetFilterRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(skip)]
    sheet_id: String,
    /// 筛选应用范围
    range: String,
    /// 设置筛选条件的列
    col: String,
    /// 筛选的条件
    condition: SheetFilterCondition,
}

impl CreateSheetFilterRequest {
    pub fn builder() -> CreateSheetFilterRequestBuilder {
        CreateSheetFilterRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateSheetFilterRequestBuilder {
    request: CreateSheetFilterRequest,
}

impl CreateSheetFilterRequestBuilder {
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

    /// 筛选应用范围
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
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

    pub fn build(mut self) -> CreateSheetFilterRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetFilterService {
    /// 在子表内创建筛选
    pub async fn create(
        &self,
        request: CreateSheetFilterRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

// 实现ExecutableBuilder trait
impl_executable_builder_owned!(
    CreateSheetFilterRequestBuilder,
    SpreadsheetSheetFilterService,
    CreateSheetFilterRequest,
    BaseResponse<EmptyResponse>,
    create
);
