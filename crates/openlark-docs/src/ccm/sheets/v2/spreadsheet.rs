
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets v2 spreadsheet - 表格操作API实现,
//,
// 实现表格级别的操作，包括：,
// - 获取表格元数据,
// - 创建表格,
// - 更新表格属性,
// - 删除表格,
// - 获取表格所有工作表,
use openlark_core::{APIResult, LarkClient, RequestBuilder};
use super::ccm::models::CcmResponse;
use serde::{Deserialize, Serialize};
/// 表格操作服务,
#[derive(Clone, Debug)]
pub struct SpreadsheetService {
    #[allow(dead_code)]
    config: Config,,
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取表格元数据,
    ///,
/// 根据 spreadsheetToken 获取表格元数据。,
    ///,
/// # 参数,
    /// - `spreadsheet_token`: 表格token,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.sheets.v2.spreadsheet.get_metadata("spreadsheet_token").await?;
/// ```,
    pub async fn get_metadata(
        &self,
        spreadsheet_token: &str,
    ) -> APIResult<CcmResponse<SpreadsheetMetadata>> {,
let request = RequestBuilder::get("/open-apis/sheets/v2/spreadsheets"),
            .query_param("spreadsheetToken", spreadsheet_token);
self.client.send(request).await,
    }
/// 获取表格所有工作表,
    ///,
/// 获取电子表格下所有工作表及其属性。,
    ///,
/// # 参数,
    /// - `spreadsheet_token`: 表格token,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.sheets.v2.spreadsheet.get_all_sheets("spreadsheet_token").await?;
/// ```,
    pub async fn get_all_sheets(
        &self,
        spreadsheet_token: &str,
    ) -> APIResult<CcmResponse<SheetListResponse>> {,
let request = RequestBuilder::get("/open-apis/sheets/v2/spreadsheets/sheets"),
            .query_param("spreadsheetToken", spreadsheet_token);
self.client.send(request).await,
    }
/// 更新表格属性,
    ///,
/// 根据 spreadsheetToken 更新表格属性，如更新表格标题。,
    ///,
/// # 参数,
    /// - `spreadsheet_token`: 表格token,
/// - `title`: 新的表格标题,
    ///,
/// # 示例,
    /// ```ignore
    /// let response = client.sheets.v2.spreadsheet.update_properties("spreadsheet_token", "新标题").await?;
/// ```,
    pub async fn update_properties(
        &self,
        spreadsheet_token: &str,
        title: &str,
    ) -> APIResult<CcmResponse<SpreadsheetMetadata>> {,
let request_body = UpdateSpreadsheetRequest {,
            title: title.to_string(),
        };
let request = RequestBuilder::put(&format!(,
            "/open-apis/sheets/v2/spreadsheets",
        ))
        .query_param()
.body_json(request_body);
        self.client.send(request).await,
/// 创建表格,
    ///,
/// 在指定目录下创建表格。,
    ///,
/// # 参数,
    /// - `folder_token`: 文件夹token,
/// - `title`: 表格标题,
    /// - `folder_type`: 文件夹类型,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.sheets.v2.spreadsheet.create("folder_token", "新表格", "explorer").await?;
/// ```,
    pub async fn create(
        &self,
        folder_token: &str,
        title: &str,
        folder_type: &str,
    ) -> APIResult<CcmResponse<SpreadsheetMetadata>> {,
let request_body = CreateSpreadsheetRequest {,
            title: title.to_string(),
            folder_token: folder_token.to_string(),
            folder_type: folder_type.to_string(),
        };
let request = RequestBuilder::post("/open-apis/sheets/v2/spreadsheets"),
            .body_json(request_body);
self.client.send(request).await,
    }
/// 删除表格,
    ///,
/// 根据 spreadsheetToken 删除对应的 sheet 文档。,
    ///,
/// # 参数,
    /// - `spreadsheet_token`: 表格token,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.sheets.v2.spreadsheet.delete("spreadsheet_token").await?;
/// ```,
    pub async fn delete(&self, spreadsheet_token: &str) -> APIResult<CcmResponse<serde_json::Value>> {,
let request = RequestBuilder::delete("/open-apis/sheets/v2/spreadsheets"),
            .query_param("spreadsheetToken", spreadsheet_token);
self.client.send(request).await,
    }
// 数据结构,
/// 表格元数据
#[derive(Clone, Debug)]
pub struct SpreadsheetMetadata {
    /// 表格token,
#[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: String,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
    /// 表格URL
    pub url: String,
    /// 是否已删除
    pub deleted: bool,
/// 创建者信息,
#[derive(Clone, Debug)]
pub struct CreatorInfo {
    /// 用户ID,
#[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户ID类型,
#[serde(rename = "user_id_type")]
    pub user_id_type: String,
    /// 用户名
    pub name: Option<String>,
/// 工作表列表响应,
#[derive(Clone, Debug)]
pub struct SheetListResponse {
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
/// 工作表信息,
#[derive(Clone, Debug)]
pub struct SheetInfo {
    /// 工作表ID,
#[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型,
#[serde(rename = "sheet_type")]
    pub sheet_type: String,
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 是否已删除
    pub deleted: bool,
// 请求结构体,
#[derive(Clone, Debug)]
pub struct UpdateSpreadsheetRequest {
    pub title: String,

#[derive(Clone, Debug)]
pub struct CreateSpreadsheetRequest {
    pub title: String,
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    #[serde(rename = "folder_type")]
    pub folder_type: String,
