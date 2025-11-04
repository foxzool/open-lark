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
    service::sheets::v3::SpreadsheetSheetService,
};
use super::create::DataValidationRule;
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新下拉列表设置请求,
#[derive(Debug, Clone)]
pub struct UpdateDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验 ID
    data_validation_id: String,
    /// 新的数据校验设置
    data_validation: DataValidationRule}
impl UpdateDataValidationRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateDataValidationRequestBuilder {
    request: UpdateDataValidationRequest}
impl UpdateDataValidationRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新下拉列表设置响应体最外层,
#[derive(Debug, Clone)]
pub struct UpdateDataValidationResponseData {
    /// 数据校验 ID
    pub data_validation_id: String,
    /// 更新后的数据校验规则信息,
#[serde(flatten)]
    pub data_validation: DataValidationRule,
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
fn test_update_data_validation_response() {
        let json = json!({
            "data_validation_id": "dv_001",
            "condition_type": "number_between",
            "range": "B1:B10",
            "condition_values": ["1", "100"]
            "strict": true,
            "input_message": "请输入1-100之间的数字",
            "error_message": "数字超出范围"});
let response: UpdateDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.data_validation_id, "dv_001");
        assert_eq!(response.data_validation.condition_type, "number_between");
