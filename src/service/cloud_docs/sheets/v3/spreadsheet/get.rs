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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetService,
};

impl SpreadsheetService {
    /// 获取电子表格信息
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get>
    pub async fn get(
        &self,
        request: GetSpreadsheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSpreadsheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_GET.replace("{}", &request.spreadsheet_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetSpreadsheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 用户 ID 类型
    ///
    /// 默认值：open_id
    ///
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    user_id_type: Option<String>,
}

impl GetSpreadsheetRequest {
    pub fn builder() -> GetSpreadsheetRequestBuilder {
        GetSpreadsheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetSpreadsheetRequestBuilder {
    request: GetSpreadsheetRequest,
}

impl GetSpreadsheetRequestBuilder {
    /// 用户 ID 类型
    ///
    /// 默认值：open_id
    ///
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店���用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 表格的token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn build(mut self) -> GetSpreadsheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    GetSpreadsheetRequestBuilder,
    SpreadsheetService,
    GetSpreadsheetRequest,
    BaseResponse<GetSpreadsheetResponseData>,
    get
);

#[derive(Debug, Deserialize)]
pub struct GetSpreadsheetResponseData {
    pub spreadsheet: GetSpreadsheetResponse,
}

impl ApiResponseTrait for GetSpreadsheetResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSpreadsheetResponse {
    /// 电子表格标题
    pub title: String,
    /// 电子表格owner
    pub owner_id: String,
    /// 电子表格token
    pub token: String,
    /// 电子表格url
    pub url: String,
}
