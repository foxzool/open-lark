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
}/// 修改保护范围请求,
#[derive(Debug, Clone)]
pub struct UpdateProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围 ID
    protect_id: String,
    /// 新的保护范围设置
    protect_range: ProtectRangeData}
impl UpdateProtectRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateProtectRangeRequestBuilder {
    request: UpdateProtectRangeRequest}
impl UpdateProtectRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 修改保护范围响应体最外层,
#[derive(Debug, Clone)]
pub struct UpdateProtectRangeResponseData {
    /// 保护范围 ID
    pub protect_id: String,
    /// 更新后的保护范围信息,
#[serde(flatten)]
    pub protect_range: ProtectRangeData,
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
fn test_update_protect_range_response() {
        let json = json!({
            "protect_id": "protect_001",
            "dimension": "ROWS",
            "sheet_id": "Sheet1",
            "start_index": 5,
            "end_index": 15,
            "lock_info": "user@example.com"});
let response: UpdateProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.protect_id, "protect_001");
        assert_eq!(response.protect_range.start_index, 5);
        assert_eq!(response.protect_range.end_index, 15);
