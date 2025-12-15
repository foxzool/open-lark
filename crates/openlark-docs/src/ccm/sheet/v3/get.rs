use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::SheetsApiV3;

/// 获取工作表信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
}

impl GetSheetRequest {
    /// 创建获取工作表信息请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = sheet_id.into();
        self
    }
}

/// 获取工作表信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for GetSheetResponse {
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

/// 获取工作表信息
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId
pub async fn get_sheet(
    request: GetSheetRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetSheetResponse>> {
    // 构建API端点URL
    let url = format!(
        "{}/spreadsheets/{}/sheets/{}",
        SheetsApiV3, request.spreadsheet_token, request.sheet_id
    );

    // 创建API请求
    let mut api_request: ApiRequest<GetSheetResponse> = ApiRequest::get(&url);

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
    fn test_get_sheet_request_builder() {
        let request = GetSheetRequest::new("spreadsheet_token", "sheet_id");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
    }

    #[test]
    fn test_get_sheet_request_with_token() {
        let request = GetSheetRequest::new("initial_token", "initial_id")
            .spreadsheet_token("new_token")
            .sheet_id("new_id");

        assert_eq!(request.spreadsheet_token, "new_token");
        assert_eq!(request.sheet_id, "new_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetSheetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_sheet_info_structure() {
        let sheet_info = serde_json::json!({
            "properties": {
                "sheet_id": "sheet_id",
                "title": "工作表标题",
                "index": 0,
                "sheet_type": "SHEET"
            }
        });

        assert!(sheet_info.get("properties").is_some());
    }
}