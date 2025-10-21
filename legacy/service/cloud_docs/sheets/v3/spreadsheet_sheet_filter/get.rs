use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::{
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService,
    },
};

/// 获取子表的详细筛选信息请求
#[derive(Debug, Default)]
pub struct SheetFilterRequest {
    api_request: ApiRequest,
    spreadsheet_token: String,
    sheet_id: String,
}

impl SheetFilterRequest {
    pub fn builder() -> SheetFilterRequestBuilder {
        SheetFilterRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SheetFilterRequestBuilder {
    request: SheetFilterRequest,
}

impl SheetFilterRequestBuilder {
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

    pub fn build(self) -> SheetFilterRequest {
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct SheetFilterResponse {
    pub sheet_filter_info: Option<SheetFilterInfo>,
}

impl ApiResponseTrait for SheetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 筛选信息
#[derive(Deserialize, Debug)]
pub struct SheetFilterInfo {
    /// 筛选应用范围
    pub range: String,
    /// 筛选出来隐藏的行
    pub filtered_out_rows: Vec<i32>,
    /// sheet的筛选条件
    pub filter_infos: Vec<FilterInfo>,
}

/// sheet的筛选条件
#[derive(Deserialize, Debug)]
pub struct FilterInfo {
    /// 设置了筛选条件的列
    pub col: String,
    /// 筛选条件
    pub conditions: Vec<SheetFilterCondition>,
}

impl SpreadsheetSheetFilterService {
    /// 获取子表的详细筛选信息
    pub async fn get(
        &self,
        request: SheetFilterRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SheetFilterResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
