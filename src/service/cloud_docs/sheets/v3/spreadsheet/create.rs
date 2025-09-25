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
    /// 创建表格
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create>
    pub async fn create(
        &self,
        request: CreateSpreedSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSpreedSheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEETS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

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

// Trait implementation
impl_executable_builder_owned!(
    CreateSpreedSheetRequestBuilder,
    SpreadsheetService,
    CreateSpreedSheetRequest,
    BaseResponse<CreateSpreedSheetResponseData>,
    create
);

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
    pub title: String,
    /// 文件夹token
    pub folder_token: String,
    /// 文档url
    pub url: String,
    /// 表格token
    pub spreadsheet_token: String,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
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
