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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除浮动图片请求,
#[derive(Debug, Clone)]
pub struct DeleteFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片 ID
    float_image_id: String}
impl DeleteFloatImageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteFloatImageRequestBuilder {
    request: DeleteFloatImageRequest}
impl DeleteFloatImageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    DeleteFloatImageRequestBuilder,
    SpreadsheetSheetService,
    DeleteFloatImageRequest,
    BaseResponse<DeleteFloatImageResponseData>,
    delete_float_image,
);
/// 删除浮动图片响应体最外层
#[derive(Debug, Clone)]
pub struct DeleteFloatImageResponseData {
    /// 删除操作是否成功,
#[serde(default)]
    pub success: bool,
    /// 删除的浮动图片 ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>}
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
fn test_delete_float_image_response() {
        let json = json!({
            "success": true,
            "float_image_id": "fimg_001"});
let response: DeleteFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.float_image_id, Some("fimg_001".to_string()));
