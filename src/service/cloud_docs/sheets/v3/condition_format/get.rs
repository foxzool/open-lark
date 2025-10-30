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
    service::sheets::v3::SpreadsheetSheetService,
};
use super::create::ConditionFormatInfo;
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 批量获取条件格式请求,
#[derive(Debug, Clone)]
pub struct GetConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 可选：查询范围，如果不提供则返回整个工作表的条件格式,
#[serde(skip_serializing_if = "Option::is_none")]
    range: Option<String>}
impl GetConditionFormatsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetConditionFormatsRequestBuilder {
    request: GetConditionFormatsRequest}
impl GetConditionFormatsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量获取条件格式响应体最外层,
#[derive(Debug, Clone)]
pub struct GetConditionFormatsResponseData {
    /// 条件格式列表
    pub items: Vec<ConditionFormatInfo>,
    /// 是否还有更多数据,
#[serde(default)]
    pub has_more: bool,
    /// 下次请求的页面标记,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use super::*;
use serde_json::json;
    #[test]
fn test_get_condition_formats_response() {
        let json = json!({,
"items": [,
                {
                    "cf_id": "cf_001",
                    "range": "A1:A10",
                    "condition_type": "NUMBER_GREATER",
                    "condition_values": ["100"]
                    "format": {
                        "background_color": "#FF0000",
                        "text_color": "#FFFFFF",
                        "bold": true}
                }
                {
                    "cf_id": "cf_002",
                    "range": "B1:B10",
                    "condition_type": "TEXT_CONTAINS",
                    "condition_values": ["重要"]
                    "format": {
                        "background_color": "#FFFF00",
                        "text_color": "#000000"}
                }
            ],
            "has_more": false,
});
let response: GetConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].cf_id, "cf_001");
assert_eq!(,
            response.items[1].condition_format.condition_type,
            "TEXT_CONTAINS",
);
        assert!(!response.has_more);
