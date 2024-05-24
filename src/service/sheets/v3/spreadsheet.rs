use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponse, ApiResponseTrait, RawResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 电子表格
pub struct SpreadsheetService {
    config: Config,
}

impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建表格
    pub fn create(
        &self,
        request: CreateSpreedSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ApiResponse<CreateSpreedSheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = "POST".to_string();
        api_req.api_path = "/open-apis/sheets/v3/spreadsheets".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

        Ok(api_resp)
    }

    /// 修改电子表格属性
    pub fn patch(
        &self,
        request: PatchSpreadSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ApiResponse<RawResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = "PATCH".to_string();
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}",
            request.spreadsheet_token
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

        Ok(api_resp)
    }

    /// 获取电子表格信息
    pub fn get(
        &self,
        request: GetSpreadsheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ApiResponse<GetSpreadsheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = "GET".to_string();
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}",
            request.spreadsheet_token
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

        Ok(api_resp)
    }
}

/// 创建表格 请求体
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateSpreedSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 表格标题
    ///
    /// 示例值："title"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 255 字符
    title: Option<String>,
    /// 文件夹token
    folder_token: Option<String>,
}

impl CreateSpreedSheetRequest {
    pub fn builder() -> CreateSpreedSheetRequestBuilder {
        CreateSpreedSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateSpreedSheetRequestBuilder {
    request: CreateSpreedSheetRequest,
}

impl CreateSpreedSheetRequestBuilder {
    /// 表格标题
    ///
    /// 示例值："title"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 255 字符
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    /// 文件夹token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.request.folder_token = Some(folder_token.to_string());
        self
    }

    pub fn build(mut self) -> CreateSpreedSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 创建表格 响应体最外层
#[derive(Deserialize, Debug)]
pub struct CreateSpreedSheetResponseData {
    pub spreadsheet: CreateSpreedSheetResponse,
}

impl ApiResponseTrait for CreateSpreedSheetResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建表格 响应体
#[derive(Deserialize, Debug)]
pub struct CreateSpreedSheetResponse {
    /// 表格标题
    title: String,
    /// 文件夹token
    folder_token: String,
    /// 文档url
    url: String,
    /// 表格token
    spreadsheet_token: String,
}

/// 修改电子表格属性 请求体
#[derive(Default, Debug, Serialize)]
pub struct PatchSpreadSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 表格的token
    #[serde(skip)]
    spreadsheet_token: String,
    /// 表格标题
    title: String,
}

impl PatchSpreadSheetRequest {
    pub fn builder() -> PatchSpreadSheetRequestBuilder {
        PatchSpreadSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PatchSpreadSheetRequestBuilder {
    request: PatchSpreadSheetRequest,
}

impl PatchSpreadSheetRequestBuilder {
    /// 表格标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = title.to_string();
        self
    }

    /// 表格的token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn build(mut self) -> PatchSpreadSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
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
    /// 示例值："open_id"
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
    /// 默认值：open_id
    ///
    /// 当值为 user_id，字段权限要求：
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
    /// 示例值："open_id"
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
    /// 默认值：open_id
    ///
    /// 当值为 user_id，字段权限要求：
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
    title: String,
    /// 电子表格owner
    owner_id: String,
    /// 电子表格token
    token: String,
    /// 电子表格url
    url: String,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::service::sheets::v3::spreadsheet::CreateSpreedSheetResponseData;

    #[test]
    fn test_create_spreadsheet_response() {
        let json = json!({
                "spreadsheet": {
                    "title": "title",
                    "folder_token": "fldcnMsNb*****hIW9IjG1LVswg",
                    "url": "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8",
                    "spreadsheet_token": "shtcnmBA*****yGehy8"
                }

        });

        let response: CreateSpreedSheetResponseData = serde_json::from_value(json).unwrap();
        let response = response.spreadsheet;

        assert_eq!(response.title, "title");
        assert_eq!(response.folder_token, "fldcnMsNb*****hIW9IjG1LVswg");
        assert_eq!(
            response.url,
            "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8"
        );
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
    }
}
