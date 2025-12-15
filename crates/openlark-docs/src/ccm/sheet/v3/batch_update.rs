/// 批量更新工作表
///
/// 批量更新多个工作表的属性，支持更新标题、索引、类型等。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets:batchUpdate
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiV3, api_utils::*};

/// 批量更新工作表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateSheetsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 批量更新请求列表
    pub requests: Vec<SheetUpdateRequest>,
    /// 是否包含电子表格响应
    pub include_spreadsheet_response: Option<bool>,
}

impl BatchUpdateSheetsRequest {
    /// 创建批量更新工作表请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            requests: vec![],
            include_spreadsheet_response: None,
        }
    }

    /// 添加更新请求
    pub fn add_request(mut self, request: SheetUpdateRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// 设置是否包含电子表格响应
    pub fn include_spreadsheet_response(mut self, include: bool) -> Self {
        self.include_spreadsheet_response = Some(include);
        self
    }
}

/// 工作表更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "updateSheetProperties")]
pub struct SheetUpdateRequest {
    /// 工作表属性
    pub properties: SheetProperties,
    /// 更新字段列表
    pub fields: Vec<String>,
}

impl SheetUpdateRequest {
    /// 创建工作表更新请求
    ///
    /// # 参数
    /// * `sheet_id` - 工作表ID
    /// * `fields` - 更新字段列表
    pub fn new(sheet_id: impl Into<String>, fields: Vec<String>) -> Self {
        Self {
            properties: SheetProperties {
                sheet_id: sheet_id.into(),
                title: None,
                index: None,
                sheet_type: None,
                grid_properties: None,
            },
            fields,
        }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.properties.title = Some(title.into());
        self
    }

    /// 设置索引
    pub fn index(mut self, index: i32) -> Self {
        self.properties.index = Some(index);
        self
    }

    /// 设置工作表类型
    pub fn sheet_type(mut self, sheet_type: impl Into<String>) -> Self {
        self.properties.sheet_type = Some(sheet_type.into());
        self
    }

    /// 设置网格属性
    pub fn grid_properties(mut self, grid_properties: GridProperties) -> Self {
        self.properties.grid_properties = Some(grid_properties);
        self
    }
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperties {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: Option<String>,
    /// 工作表索引
    pub index: Option<i32>,
    /// 工作表类型
    pub sheet_type: Option<String>,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 行数
    pub row_count: Option<i32>,
    /// 列数
    pub column_count: Option<i32>,
    /// 冻结行数
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    pub frozen_column_count: Option<i32>,
}

/// 批量更新工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateSheetsResponse {
    /// 批量更新结果
    pub data: Option<BatchUpdateResult>,
}

impl ApiResponseTrait for BatchUpdateSheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateResult {
    /// 电子表格属性
    pub spreadsheet: Option<SpreadsheetProperties>,
    /// 更新结果列表
    pub replies: Vec<SheetUpdateResponse>,
}

/// 工作表更新响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetUpdateResponse {
    /// 工作表属性
    pub properties: Option<SheetPropertiesInfo>,
}

/// 电子表格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
}

/// 工作表属性信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetPropertiesInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
    /// 网格属性
    pub grid_properties: Option<GridPropertiesInfo>,
}

/// 网格属性信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPropertiesInfo {
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 冻结行数
    pub frozen_row_count: i32,
    /// 冻结列数
    pub frozen_column_count: i32,
}

/// 批量更新工作表
///
/// 批量更新多个工作表的属性，支持更新标题、索引、类型等。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets:batchUpdate
pub async fn batch_update_sheets(
    request: BatchUpdateSheetsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<BatchUpdateSheetsResponse>> {
    // 构建请求体
    let mut body = json!({
        "requests": request.requests
    });

    if let Some(include) = request.include_spreadsheet_response {
        body["includeSpreadsheetResponse"] = json!(include);
    }

    // 创建API请求
    let mut api_request: ApiRequest<BatchUpdateSheetsResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/sheets:batchUpdate", CcmSheetApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_update_sheets_request_builder() {
        let request = BatchUpdateSheetsRequest::new("spreadsheet_token");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.requests.is_empty());
        assert!(request.include_spreadsheet_response.is_none());
    }

    #[test]
    fn test_batch_update_sheets_request_builder_chain() {
        let update_request1 = SheetUpdateRequest::new("sheet1", vec!["title".to_string()])
            .title("新标题1");

        let update_request2 = SheetUpdateRequest::new("sheet2", vec!["title".to_string(), "index".to_string()])
            .title("新标题2")
            .index(1);

        let request = BatchUpdateSheetsRequest::new("spreadsheet_token")
            .add_request(update_request1)
            .add_request(update_request2)
            .include_spreadsheet_response(true);

        assert_eq!(request.requests.len(), 2);
        assert_eq!(request.include_spreadsheet_response, Some(true));
    }

    #[test]
    fn test_sheet_update_request_builder() {
        let request = SheetUpdateRequest::new("sheet_id", vec!["title".to_string()])
            .title("新标题")
            .index(1)
            .sheet_type("GRID");

        assert_eq!(request.properties.sheet_id, "sheet_id");
        assert_eq!(request.properties.title, Some("新标题".to_string()));
        assert_eq!(request.properties.index, Some(1));
        assert_eq!(request.properties.sheet_type, Some("GRID".to_string()));
        assert_eq!(request.fields, vec!["title"]);
    }

    #[test]
    fn test_sheet_properties_info_structure() {
        let sheet_info = SheetPropertiesInfo {
            sheet_id: "sheet_123".to_string(),
            title: "工作表1".to_string(),
            index: 0,
            sheet_type: "GRID".to_string(),
            grid_properties: None,
        };

        assert_eq!(sheet_info.sheet_id, "sheet_123");
        assert_eq!(sheet_info.title, "工作表1");
        assert_eq!(sheet_info.index, 0);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(BatchUpdateSheetsResponse::data_format(), ResponseFormat::Data);
    }
}