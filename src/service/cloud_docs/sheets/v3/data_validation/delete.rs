#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除下拉列表设置请求,
#[derive(Debug, Clone)]
pub struct DeleteDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验 ID
    data_validation_id: String}
impl DeleteDataValidationRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteDataValidationRequestBuilder {
    request: DeleteDataValidationRequest}
impl DeleteDataValidationRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除下拉列表设置响应体最外层,
#[derive(Debug, Clone)]
pub struct DeleteDataValidationResponseData {
    /// 删除操作是否成功,
#[serde(default)]
    pub success: bool,
    /// 删除的数据校验 ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub data_validation_id: Option<String>}
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
fn test_delete_data_validation_response() {
        let json = json!({
            "success": true,
            "data_validation_id": "dv_001"});
let response: DeleteDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.data_validation_id, Some("dv_001".to_string()));
