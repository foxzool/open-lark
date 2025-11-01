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

use super::create::{ConditionFormatInfo, ConditionFormatRule};
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量更新条件格式请求,
#[derive(Debug, Clone)]
pub struct UpdateConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 要更新的条件格式规则列表
    condition_formats: Vec<UpdateConditionFormatRule>}
impl UpdateConditionFormatsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateConditionFormatsRequestBuilder {
    request: UpdateConditionFormatsRequest}
impl UpdateConditionFormatsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新条件格式规则,
#[derive(Debug, Clone)]
pub struct UpdateConditionFormatRule {
    /// 条件格式 ID
    pub cf_id: String,
    /// 条件格式规则,
#[serde(flatten)]
    pub rule: ConditionFormatRule,
impl UpdateConditionFormatRule {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量更新条件格式响应体最外层,
#[derive(Debug, Clone)]
pub struct UpdateConditionFormatsResponseData {
    /// 更新后的条件格式列表
    pub items: Vec<ConditionFormatInfo>,
    /// 更新成功的数量,
#[serde(default)]
    pub updated_count: u32,
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
fn test_update_condition_formats_response() {
        let json = json!({,
"items": [,
                {
                    "cf_id": "cf_001",
                    "range": "A1:A15",
                    "condition_type": "NUMBER_GREATER",
                    "condition_values": ["200"]
                    "format": {
                        "background_color": "#00FF00",
                        "text_color": "#000000",
                        "bold": false}
                }
            ],
            "updated_count": 1,
});
let response: UpdateConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].cf_id, "cf_001");
        assert_eq!(response.updated_count, 1);
