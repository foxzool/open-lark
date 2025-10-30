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
    service::sheets::v3::SpreadsheetService,
};
use super::create::ProtectRangeData;
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 获取保护范围请求,
#[derive(Debug, Clone)]
pub struct GetProtectRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 可选：sheet 的 id，如果不提供则返回所有工作表的保护范围,
#[serde(skip_serializing_if = "Option::is_none")]
    sheet_id: Option<String>}
impl GetProtectRangesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetProtectRangesRequestBuilder {
    request: GetProtectRangesRequest}
impl GetProtectRangesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 保护范围信息,
#[derive(Debug, Clone)]
pub struct ProtectRangeInfo {
    /// 保护范围 ID
    pub protect_id: String,
    /// 保护范围详细信息,
#[serde(flatten)]
    pub protect_range: ProtectRangeData,
/// 获取保护范围响应体最外层,
#[derive(Debug, Clone)]
pub struct GetProtectRangesResponseData {
    /// 保护范围列表
    pub items: Vec<ProtectRangeInfo>,
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
fn test_get_protect_ranges_response() {
        let json = json!({,
"items": [,
                {
                    "protect_id": "protect_001",
                    "dimension": "ROWS",
                    "sheet_id": "Sheet1",
                    "start_index": 1,
                    "end_index": 10,
                    "lock_info": "user1@example.com"}
                {
                    "protect_id": "protect_002",
                    "dimension": "COLUMNS",
                    "sheet_id": "Sheet1",
                    "start_index": 1,
                    "end_index": 5,
                    "lock_info": "user2@example.com"}
            ],
            "has_more": false,
});
let response: GetProtectRangesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].protect_id, "protect_001");
        assert_eq!(response.items[1].protect_range.dimension, "COLUMNS");
assert!(!response.has_more);
    }
