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
}/// 创建浮动图片请求,
#[derive(Debug, Clone)]
pub struct CreateFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片信息
    float_image: FloatImageData}
impl CreateFloatImageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateFloatImageRequestBuilder {
    request: CreateFloatImageRequest}
impl CreateFloatImageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateFloatImageRequestBuilder,
    SpreadsheetSheetService,
    CreateFloatImageRequest,
    BaseResponse<CreateFloatImageResponseData>,
    create_float_image,
);
/// 浮动图片数据
#[derive(Debug, Clone)]
pub struct FloatImageData {
    /// 图片token
    pub image_token: String,
    /// 图片位置
    pub position: ImagePosition,
    /// 图片大小
    pub size: ImageSize,
    /// 浮动图片 ID（仅在响应时存在）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 图片名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>}
/// 图片位置,
#[derive(Debug, Clone)]
pub struct ImagePosition {
    /// 起始列索引（从0开始）
    pub start_col_index: i32,
    /// 起始行索引（从0开始）
    pub start_row_index: i32,
    /// 在单元格内的水平偏移量（像素）,
#[serde(default)]
    pub offset_x: f64,
    /// 在单元格内的垂直偏移量（像素）,
#[serde(default)]
    pub offset_y: f64,
/// 图片大小,
#[derive(Debug, Clone)]
pub struct ImageSize {
    /// 图片宽度（像素）
    pub width: f64,
    /// 图片高度（像素）
    pub height: f64,
impl FloatImageData {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置图片名称,
    pub fn with_name(mut self, name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.name = Some(name.to_string());
        self}
impl ImagePosition {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置偏移量,
    pub fn with_offset(mut self, offset_x: f64, offset_y: f64) -> Self {
self.offset_x = offset_x;
        self.offset_y = offset_y;
self}
impl ImageSize {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建正方形图片大小,
    pub fn square(size: f64) -> Self {
Self {
            width: size,
            height: size}
/// 创建浮动图片响应体最外层,
#[derive(Debug, Clone)]
pub struct CreateFloatImageResponseData {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 浮动图片信息,
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
fn test_float_image_creation() {
        let position = ImagePosition::new(1, 1).with_offset(10.0, 20.0);
        let size = ImageSize::new(200.0, 150.0);
let float_image =,
            FloatImageData::new("img_token_123", position, size).with_name("示例图片");

        assert_eq!(float_image.image_token, "img_token_123");
        assert_eq!(float_image.position.start_col_index, 1);
        assert_eq!(float_image.position.offset_x, 10.0);
        assert_eq!(float_image.size.width, 200.0);
        assert_eq!(float_image.name.as_ref().unwrap(), "示例图片");
#[test]
    fn test_create_float_image_response() {
let json = json!({,
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
let response: CreateFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image_id, "fimg_001");
        assert_eq!(response.float_image.image_token, "img_token_123");
