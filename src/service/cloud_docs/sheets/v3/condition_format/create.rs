#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
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
}/// 批量创建条件格式请求,
#[derive(Debug, Clone)]
pub struct CreateConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 条件格式规则列表
    condition_formats: Vec<ConditionFormatRule>}
impl CreateConditionFormatsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateConditionFormatsRequestBuilder {
    request: CreateConditionFormatsRequest}
impl CreateConditionFormatsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 条件格式规则,
#[derive(Debug, Clone)]
pub struct ConditionFormatRule {
    /// 应用范围
    pub range: String,
    /// 条件类型
    pub condition_type: String,
    /// 条件值,
#[serde(skip_serializing_if = "Option::is_none")]
    pub condition_values: Option<Vec<String>>,
    /// 格式设置
    pub format: FormatStyle,
    /// 条件格式 ID（仅在响应时存在）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub cf_id: Option<String>}
/// 格式样式,
#[derive(Debug, Clone)]
pub struct FormatStyle {
    /// 背景颜色,
#[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// 文字颜色,
#[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// 是否加粗,
#[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 是否斜体,
#[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 是否下划线,
#[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    /// 是否删除线,
#[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>}
impl ConditionFormatRule {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建大于条件格式,
    pub fn greater_than(range: impl ToString, value: f64, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::number_comparison(range, "NUMBER_GREATER", value, format)}
/// 创建小于条件格式,
    pub fn less_than(range: impl ToString, value: f64, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::number_comparison(range, "NUMBER_LESS", value, format)}
/// 创建等于条件格式,
    pub fn equal_to(range: impl ToString, value: f64, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::number_comparison(range, "NUMBER_EQ", value, format)}
/// 创建文本包含条件格式,
    pub fn text_contains(range: impl ToString, text: impl ToString, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            range: range.to_string(),
            condition_type: "TEXT_CONTAINS".to_string(),
            condition_values: Some(vec![text.to_string()]),
            format,
            cf_id: None}
/// 创建重复值条件格式,
    pub fn duplicate_values(range: impl ToString, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            range: range.to_string(),
            condition_type: "DUPLICATE".to_string(),
            condition_values: None,
            format,
            cf_id: None}
/// 创建空值条件格式,
    pub fn blank_values(range: impl ToString, format: FormatStyle) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            range: range.to_string(),
            condition_type: "BLANK".to_string(),
            condition_values: None,
            format,
            cf_id: None}
impl FormatStyle {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建文字颜色格式,
    pub fn text_color(color: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            background_color: None,
            text_color: Some(color.to_string()),
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None}
/// 创建字体样式格式,
    pub fn font_style(bold: bool, italic: bool, underline: bool) -> Self {
Self {
            background_color: None,
            text_color: None,
            bold: Some(bold),
            italic: Some(italic),
            underline: Some(underline),
            strikethrough: None}
/// 设置背景颜色,
    pub fn with_background_color(mut self, color: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.background_color = Some(color.to_string());
        self}
/// 设置文字颜色,
    pub fn with_text_color(mut self, color: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.text_color = Some(color.to_string());
        self}
/// 设置加粗,
    pub fn with_bold(mut self, bold: bool) -> Self {
self.bold = Some(bold);
        self}
/// 设置斜体,
    pub fn with_italic(mut self, italic: bool) -> Self {
self.italic = Some(italic);
        self}
/// 设置下划线,
    pub fn with_underline(mut self, underline: bool) -> Self {
self.underline = Some(underline);
        self}
/// 设置删除线,
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
self.strikethrough = Some(strikethrough);
        self}
/// 条件格式信息,
#[derive(Debug, Clone)]
pub struct ConditionFormatInfo {
    /// 条件格式 ID
    pub cf_id: String,
    /// 条件格式规则详细信息,
#[serde(flatten)]
    pub condition_format: ConditionFormatRule,
/// 批量创建条件格式响应体最外层,
#[derive(Debug, Clone)]
pub struct CreateConditionFormatsResponseData {
    /// 创建的条件格式列表
    pub items: Vec<ConditionFormatInfo>,
    /// 创建成功的数量,
#[serde(default)]
    pub created_count: u32,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl_executable_builder_owned!(,
    CreateConditionFormatsRequestBuilder,
    SpreadsheetSheetService,
    CreateConditionFormatsRequest,
    BaseResponse<CreateConditionFormatsResponseData>,
    create_condition_formats,
);
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use super::*;
use serde_json::json;
    #[test]
fn test_condition_format_rule_creation() {
        let format = FormatStyle::background_color("#FF0000").with_text_color("#FFFFFF");
        let rule = ConditionFormatRule::greater_than("A1:A10", 100.0, format);

        assert_eq!(rule.range, "A1:A10");
        assert_eq!(rule.condition_type, "NUMBER_GREATER");
        assert_eq!(rule.condition_values.as_ref().unwrap()[0] "100");
        assert_eq!(rule.format.background_color.as_ref().unwrap(), "#FF0000");
        assert_eq!(rule.format.text_color.as_ref().unwrap(), "#FFFFFF");
#[test]
    fn test_create_condition_formats_response() {
let json = json!({,
            "items": [,
{,
                    "cf_id": "cf_001",
                    "range": "A1:A10",
                    "condition_type": "NUMBER_GREATER",
                    "condition_values": ["100"]
                    "format": {
                        "background_color": "#FF0000",
                        "text_color": "#FFFFFF",
                        "bold": true}
                }
            ],
            "created_count": 1,
});
let response: CreateConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].cf_id, "cf_001");
        assert_eq!(response.created_count, 1);
