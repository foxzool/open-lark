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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置下拉列表请求,
#[derive(Debug, Clone)]
pub struct SetDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验设置
    data_validation: DataValidationRule}
impl SetDataValidationRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct SetDataValidationRequestBuilder {
    request: SetDataValidationRequest}
impl SetDataValidationRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 数据校验规则,
#[derive(Debug, Clone)]
pub struct DataValidationRule {
    /// 数据校验类型
    pub condition_type: String,
    /// 应用范围
    pub range: String,
    /// 校验条件值,
#[serde(skip_serializing_if = "Option::is_none")]
    pub condition_values: Option<Vec<String>>,
    /// 是否拒绝输入,
#[serde(default)]
    pub strict: bool,
    /// 输入提示消息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub input_message: Option<String>,
    /// 错误提示消息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 数据校验 ID（仅在响应时存在）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub data_validation_id: Option<String>}
impl DataValidationRule {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建数字范围校验,
    pub fn number_range(range: impl ToString, min: f64, max: f64) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            condition_type: "number_between".to_string(),
            range: range.to_string(),
            condition_values: Some(vec![min.to_string(), max.to_string()]),
            strict: true,
            input_message: None,
            error_message: None,
            data_validation_id: None}
/// 创建文本长度校验,
    pub fn text_length(range: impl ToString, min_length: u32, max_length: u32) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            condition_type: "text_length".to_string(),
            range: range.to_string(),
            condition_values: Some(vec![min_length.to_string(), max_length.to_string()]),
            strict: true,
            input_message: None,
            error_message: None,
            data_validation_id: None}
/// 设置输入提示,
    pub fn with_input_message(mut self, message: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.input_message = Some(message.to_string());
        self}
/// 设置错误提示,
    pub fn with_error_message(mut self, message: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.error_message = Some(message.to_string());
        self}
/// 设置是否严格模式,
    pub fn with_strict(mut self, strict: bool) -> Self {
self.strict = strict;
        self}
/// 设置下拉列表响应体最外层,
#[derive(Debug, Clone)]
pub struct SetDataValidationResponseData {
    /// 数据校验 ID
    pub data_validation_id: String,
    /// 数据校验规则信息,
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
fn test_data_validation_rule_creation() {
        let validation =
            DataValidationRule::dropdown("A1:A10", vec!["选项1".to_string(), "选项2".to_string()]);
        assert_eq!(validation.condition_type, "dropdown");
        assert_eq!(validation.range, "A1:A10");
assert!(validation.strict);
        assert_eq!(validation.condition_values.as_ref().unwrap().len(), 2);
#[test]
    fn test_set_data_validation_response() {
let json = json!({,
            "data_validation_id": "dv_001",
            "condition_type": "dropdown",
            "range": "A1:A10",
            "condition_values": ["选项1", "选项2"]
            "strict": true,
            "input_message": "请选择一个选项",
            "error_message": "输入无效"});
let response: SetDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.data_validation_id, "dv_001");
        assert_eq!(response.data_validation.condition_type, "dropdown");
// 实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    SetDataValidationRequestBuilder,
    SpreadsheetSheetService,
    SetDataValidationRequest,
    BaseResponse<SetDataValidationResponseData>,
    set_data_validation,
);
