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

use super::query::FilterConditionInfo;

impl SpreadsheetSheetFilterViewService {
    /// 获取筛选条件
    pub async fn get_condition(
        &self,
        request: GetFilterViewConditionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFilterViewConditionResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.filter_view_id)
            .replace("{}", &request.condition_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 获取筛选条件请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetFilterViewConditionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选条件 ID
    condition_id: String,
}

impl GetFilterViewConditionRequest {
    pub fn builder() -> GetFilterViewConditionRequestBuilder {
        GetFilterViewConditionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetFilterViewConditionRequestBuilder {
    request: GetFilterViewConditionRequest,
}

impl GetFilterViewConditionRequestBuilder {
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

    pub fn condition_id(mut self, condition_id: impl ToString) -> Self {
        self.request.condition_id = condition_id.to_string();
        self
    }

    pub fn build(mut self) -> GetFilterViewConditionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 获取筛选条件响应体最外层
#[derive(Deserialize, Debug)]
pub struct GetFilterViewConditionResponseData {
    /// 筛选条件信息
    #[serde(flatten)]
    pub condition_info: FilterConditionInfo,
}

impl ApiResponseTrait for GetFilterViewConditionResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
