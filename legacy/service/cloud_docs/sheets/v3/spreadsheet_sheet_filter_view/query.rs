use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};

impl SpreadsheetSheetFilterViewService {
    /// 查询筛选视图
    pub async fn query(
        &self,
        request: QueryFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryFilterViewResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER_VIEWS
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 查询筛选视图请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct QueryFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
}

impl QueryFilterViewRequest {
    pub fn builder() -> QueryFilterViewRequestBuilder {
        QueryFilterViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryFilterViewRequestBuilder {
    request: QueryFilterViewRequest,
}

impl QueryFilterViewRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn build(mut self) -> QueryFilterViewRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 查询筛选视图响应体最外层
#[derive(Deserialize, Debug)]
pub struct QueryFilterViewResponseData {
    /// 筛选视图列表
    pub items: Vec<FilterViewInfo>,
}

impl ApiResponseTrait for QueryFilterViewResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 筛选视图信息
#[derive(Deserialize, Debug)]
pub struct FilterViewInfo {
    /// 筛选视图 ID
    pub filter_view_id: String,
    /// 筛选视图名称
    pub filter_view_name: String,
    /// 筛选范围
    pub range: String,
}
