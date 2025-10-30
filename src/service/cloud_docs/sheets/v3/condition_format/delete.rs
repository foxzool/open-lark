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
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量删除条件格式请求,
#[derive(Debug, Clone)]
pub struct DeleteConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 要删除的条件格式 ID 列表
    cf_ids: Vec<String>}
impl DeleteConditionFormatsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteConditionFormatsRequestBuilder {
    request: DeleteConditionFormatsRequest}
impl DeleteConditionFormatsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除结果,
#[derive(Debug, Clone)]
pub struct DeleteResult {
    /// 条件格式 ID
    pub cf_id: String,
    /// 删除是否成功,
#[serde(default)]
    pub success: bool,
    /// 错误信息（如果删除失败）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>}
/// 批量删除条件格式响应体最外层,
#[derive(Debug, Clone)]
pub struct DeleteConditionFormatsResponseData {
    /// 删除结果列表
    pub items: Vec<DeleteResult>,
    /// 删除成功的数量,
#[serde(default)]
    pub deleted_count: u32,
    /// 删除失败的数量,
#[serde(default)]
    pub failed_count: u32,
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
fn test_delete_condition_formats_response() {
        let json = json!({,
"items": [,
                {
                    "cf_id": "cf_001",
                    "success": true}
                {
                    "cf_id": "cf_002",
                    "success": false,
                    "error_message": "条件格式不存在"}
            ],
            "deleted_count": 1,
            "failed_count": 1,
});
let response: DeleteConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
assert!(response.items[0].success);
        assert!(!response.items[1].success);
        assert_eq!(response.deleted_count, 1);
        assert_eq!(response.failed_count, 1);
