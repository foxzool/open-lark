#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
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
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除保护范围请求,
#[derive(Debug, Clone)]
pub struct DeleteProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围 ID
    protect_id: String}
impl DeleteProtectRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteProtectRangeRequestBuilder {
    request: DeleteProtectRangeRequest}
impl DeleteProtectRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除保护范围响应体最外层,
#[derive(Debug, Clone)]
pub struct DeleteProtectRangeResponseData {
    /// 删除操作是否成功,
#[serde(default)]
    pub success: bool,
    /// 删除的保护范围 ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub protect_id: Option<String>}
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
fn test_delete_protect_range_response() {
        let json = json!({
            "success": true,
            "protect_id": "protect_001"});
let response: DeleteProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.protect_id, Some("protect_001".to_string()));
