use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponse, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::core::api_resp::RawResponse;

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
        api_req.api_path = format!("/open-apis/sheets/v3/spreadsheets/{}", request.spreadsheet_token);
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
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSpreedSheetResponseData {
    pub spreadsheet: CreateSpreedSheetResponse,
}

impl ApiResponseTrait for CreateSpreedSheetResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建表格 响应体
#[derive(Serialize, Deserialize, Debug)]
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
