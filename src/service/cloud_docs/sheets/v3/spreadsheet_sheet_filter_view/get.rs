use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};

use super::query::FilterViewInfo;

impl SpreadsheetSheetFilterViewService {
    /// 获取筛选视图
    pub async fn get(
        &self,
        request: GetFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFilterViewResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
            request.spreadsheet_token, request.sheet_id, request.filter_view_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 获取筛选视图请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
}

impl GetFilterViewRequest {
    pub fn builder() -> GetFilterViewRequestBuilder {
        GetFilterViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetFilterViewRequestBuilder {
    request: GetFilterViewRequest,
}

impl GetFilterViewRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn filter_view_id(mut self, filter_view_id: impl ToString) -> Self {
        self.request.filter_view_id = filter_view_id.to_string();
        self
    }

    pub fn build(mut self) -> GetFilterViewRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 获取筛选视图响应体最外层
#[derive(Deserialize, Debug)]
pub struct GetFilterViewResponseData {
    /// 筛选视图信息
    #[serde(flatten)]
    pub filter_view: FilterViewInfo,
}

impl ApiResponseTrait for GetFilterViewResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
