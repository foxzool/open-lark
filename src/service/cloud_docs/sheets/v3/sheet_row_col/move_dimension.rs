#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use serde::Serialize;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
core::{,
        api_resp::{BaseResponse, EmptyResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    }
    service::sheets::v3::SpreadsheetSheetService,
};
/// 移动行列请求,
#[derive(Debug, Clone)]
pub struct MoveDimensionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 电子表格的 token,
#[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的 ID。调用获取工作表获取 ID,
#[serde(skip)]
    sheet_id: String,
    /// 移动源位置信息
    source: Dimension,
    /// 移动的目标位置行或者列
    destination_index: Option<i32>}

#[derive(Debug, Clone)]
struct Dimension {,
    /// 更新的维度。可选值：,
/// - ROWS：行,
    /// - COLUMNS：列
    major_dimension: String,
    /// 要移动的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4,
/// 行或列开始移动。包含第 4 行或列。,
    start_index: i32,
    /// 要移动的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则要移动的范围至第 8,
/// 行或列结束。不包含第 8 行或列。,
    ///,
/// 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7 时，则移动第 4、5、6、7,
    /// 行，共 4 行。
    end_index: i32}
impl MoveDimensionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct MoveDimensionRequestBuilder {
    request: MoveDimensionRequest}
impl MoveDimensionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}