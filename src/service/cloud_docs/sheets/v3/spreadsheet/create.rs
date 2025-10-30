use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetService,
};
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建表格 请求体,
#[derive(Debug, Clone)]
pub struct CreateSpreedSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 表格标题,
///,
    /// 示例值："title",
///,
    /// 数据校验规则：,
///,
    /// 长度范围：0 字符 ～ 255 字符
    title: Option<String>,
    /// 文件夹token
    folder_token: Option<String>}
impl CreateSpreedSheetRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateSpreedSheetRequestBuilder {
    request: CreateSpreedSheetRequest}
impl CreateSpreedSheetRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    CreateSpreedSheetRequestBuilder,
    SpreadsheetService,
    CreateSpreedSheetRequest,
    BaseResponse<CreateSpreedSheetResponseData>,
    create,
);
/// 创建表格 响应体最外层
#[derive(Debug, Clone)]
pub struct CreateSpreedSheetResponseData {
    pub spreadsheet: CreateSpreedSheetResponse,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 创建表格 响应体,
#[derive(Debug, Clone)]
pub struct CreateSpreedSheetResponse {
    /// 表格标题
    pub title: String,
    /// 文件夹token
    pub folder_token: String,
    /// 文档url
    pub url: String,
    /// 表格token
    pub spreadsheet_token: String,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use crate::service::sheets::v3::spreadsheet::CreateSpreedSheetResponseData;
    #[test]
fn test_create_spreadsheet_response() {
        let json = json!({,
"spreadsheet": {,
                    "title": "title",
                    "folder_token": "fldcnMsNb*****hIW9IjG1LVswg",
                    "url": "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8",
                    "spreadsheet_token": "shtcnmBA*****yGehy8"}
,
});
let response: CreateSpreedSheetResponseData = serde_json::from_value(json).unwrap();
        let response = response.spreadsheet;

        assert_eq!(response.title, "title");
        assert_eq!(response.folder_token, "fldcnMsNb*****hIW9IjG1LVswg");
assert_eq!(,
            response.url,
            "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8",
);
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
