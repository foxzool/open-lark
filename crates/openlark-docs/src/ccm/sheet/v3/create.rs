/// 创建工作表
///
/// 在电子表格中创建新的工作表，支持设置工作表属性。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiV3, api_utils::*};

/// 创建工作表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表属性
    pub properties: SheetProperties,
}

impl CreateSheetRequest {
    /// 创建创建工作表请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `title` - 工作表标题
    pub fn new(
        spreadsheet_token: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            properties: SheetProperties {
                title: title.into(),
                index: None,
                sheet_id: None,
                sheet_type: None,
                grid_properties: None,
            },
        }
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> Self {
        self.properties.index = Some(index);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.properties.sheet_id = Some(sheet_id.into());
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
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: Option<i32>,
    /// 工作表ID
    pub sheet_id: Option<String>,
    /// 工作表类型
    pub sheet_type: Option<String>,
    /// 网格线是否显示
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 是否冻结行
    pub frozen_row_count: Option<i32>,
    /// 是否冻结列
    pub frozen_column_count: Option<i32>,
}

impl GridProperties {
    /// 创建网格属性
    ///
    /// # 参数
    /// * `row_count` - 行数
    /// * `column_count` - 列数
    pub fn new(row_count: i32, column_count: i32) -> Self {
        Self {
            row_count,
            column_count,
            frozen_row_count: None,
            frozen_column_count: None,
        }
    }

    /// 设置冻结行数
    pub fn frozen_row_count(mut self, frozen_row_count: i32) -> Self {
        self.frozen_row_count = Some(frozen_row_count);
        self
    }

    /// 设置冻结列数
    pub fn frozen_column_count(mut self, frozen_column_count: i32) -> Self {
        self.frozen_column_count = Some(frozen_column_count);
        self
    }
}

/// 创建工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSheetResponse {
    /// 电子表格属性
    pub data: Option<SpreadsheetProperties>,
}

impl ApiResponseTrait for CreateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
    /// 数据区域
    pub data: Option<Vec<GridData>>,
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

/// 网格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellData {
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 单元格值
    pub value: Option<serde_json::Value>,
    /// 格式
    pub format: Option<String>,
}

/// 创建工作表
///
/// 在电子表格中创建新的工作表，支持设置工作表属性。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets
pub async fn create_sheet(
    request: CreateSheetRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateSheetResponse>> {
    // 构建请求体
    let body = json!({
        "properties": request.properties
    });

    // 创建API请求
    let mut api_request: ApiRequest<CreateSheetResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/sheets", CcmSheetApiV3, request.spreadsheet_token))
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
    fn test_create_sheet_request_builder() {
        let request = CreateSheetRequest::new("spreadsheet_token", "新工作表");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.properties.title, "新工作表");
        assert!(request.properties.index.is_none());
    }

    #[test]
    fn test_create_sheet_request_builder_chain() {
        let grid_properties = GridProperties::new(1000, 26)
            .frozen_row_count(1)
            .frozen_column_count(0);

        let request = CreateSheetRequest::new("spreadsheet_token", "新工作表")
            .index(0)
            .sheet_type("GRID")
            .grid_properties(grid_properties);

        assert_eq!(request.properties.index, Some(0));
        assert_eq!(request.properties.sheet_type, Some("GRID".to_string()));
        assert!(request.properties.grid_properties.is_some());
    }

    #[test]
    fn test_grid_properties_builder() {
        let grid_properties = GridProperties::new(100, 20)
            .frozen_row_count(2)
            .frozen_column_count(1);

        assert_eq!(grid_properties.row_count, 100);
        assert_eq!(grid_properties.column_count, 20);
        assert_eq!(grid_properties.frozen_row_count, Some(2));
        assert_eq!(grid_properties.frozen_column_count, Some(1));
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
        assert_eq!(CreateSheetResponse::data_format(), ResponseFormat::Data);
    }
}