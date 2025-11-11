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
use super::create::FloatImageData;
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取浮动图片请求,
#[derive(Debug, Clone)]
pub struct GetFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片 ID
    float_image_id: String}
impl GetFloatImageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetFloatImageRequestBuilder {
    request: GetFloatImageRequest}
impl GetFloatImageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetFloatImageRequestBuilder,
    SpreadsheetSheetService,
    GetFloatImageRequest,
    BaseResponse<GetFloatImageResponseData>,
    get_float_image,
);
/// 获取浮动图片响应体最外层
#[derive(Debug, Clone)]
pub struct GetFloatImageResponseData {
    /// 浮动图片信息,
#[serde(flatten)]
    pub float_image: FloatImageInfo,
/// 浮动图片信息,
#[derive(Debug, Clone)]
pub struct FloatImageInfo {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 浮动图片详细信息,
#[serde(flatten)]
    pub float_image: FloatImageData,
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
fn test_get_float_image_response() {
        let json = json!({
            "float_image_id": "fimg_001",
            "image_token": "img_token_123",
            "position": {
                "start_col_index": 1,
                "start_row_index": 1,
                "offset_x": 10.0,
                "offset_y": 20.0}
            "size": {
                "width": 200.0,
                "height": 150.0}
            "name": "示例图片",
});
let response: GetFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image.float_image_id, "fimg_001");
assert_eq!(,
            response.float_image.float_image.image_token,
            "img_token_123",
);
    }
