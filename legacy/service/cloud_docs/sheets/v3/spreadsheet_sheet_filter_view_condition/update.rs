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

use super::create::FilterCondition;

impl SpreadsheetSheetFilterViewService {
    /// 更新筛选条件
    pub async fn update_condition(
        &self,
        request: UpdateFilterViewConditionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateFilterViewConditionResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
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

/// 更新筛选条件请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateFilterViewConditionRequest {
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
    /// 新的筛选条件
    condition: FilterCondition,
}

impl UpdateFilterViewConditionRequest {
    pub fn builder() -> UpdateFilterViewConditionRequestBuilder {
        UpdateFilterViewConditionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateFilterViewConditionRequestBuilder {
    request: UpdateFilterViewConditionRequest,
}

impl UpdateFilterViewConditionRequestBuilder {
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

    pub fn condition(mut self, condition: FilterCondition) -> Self {
        self.request.condition = condition;
        self
    }

    pub fn build(mut self) -> UpdateFilterViewConditionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新筛选条件响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateFilterViewConditionResponseData {
    /// 筛选条件 ID
    pub condition_id: String,
    /// 更新后的筛选条件
    pub condition: FilterCondition,
}

impl ApiResponseTrait for UpdateFilterViewConditionResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
