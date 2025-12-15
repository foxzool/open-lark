use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::api_endpoints::SheetsApiV3;

/// 更新工作表属性请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表属性
    pub properties: SheetProperties,
    /// 更新字段列表
    pub fields: Vec<String>,
}

impl UpdateSheetRequest {
    /// 创建更新工作表属性请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    /// * `properties` - 工作表属性
    /// * `fields` - 更新字段列表
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        properties: SheetProperties,
        fields: Vec<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            properties,
            fields,
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

    /// 设置工作表属性
    pub fn properties(mut self, properties: SheetProperties) -> Self {
        self.properties = properties;
        self
    }

    /// 设置更新字段列表
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperties {
    /// 工作表标题
    pub title: Option<String>,
    /// 工作表索引
    pub index: Option<i32>,
    /// 工作表类型
    pub sheet_type: Option<String>,
    /// 网格线是否显示
    pub grid_properties: Option<GridProperties>,
}

impl SheetProperties {
    /// 创建工作表属性
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置工作表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }

    /// 设置工作表类型
    pub fn sheet_type(mut self, sheet_type: impl Into<String>) -> Self {
        self.sheet_type = Some(sheet_type.into());
        self
    }

    /// 设置网格属性
    pub fn grid_properties(mut self, grid_properties: GridProperties) -> Self {
        self.grid_properties = Some(grid_properties);
        self
    }
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 行数
    pub row_count: Option<i32>,
    /// 列数
    pub column_count: Option<i32>,
    /// 是否冻结行
    pub frozen_row_count: Option<i32>,
    /// 是否冻结列
    pub frozen_column_count: Option<i32>,
}

impl GridProperties {
    /// 创建网格属性
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置行数
    pub fn row_count(mut self, row_count: i32) -> Self {
        self.row_count = Some(row_count);
        self
    }

    /// 设置列数
    pub fn column_count(mut self, column_count: i32) -> Self {
        self.column_count = Some(column_count);
        self
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

/// 更新工作表属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for UpdateSheetResponse {
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

/// 更新工作表属性
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId
pub async fn update_sheet(
    request: UpdateSheetRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateSheetResponse>> {
    // 构建API端点URL
    let url = format!(
        "{}/spreadsheets/{}/sheets/{}",
        SheetsApiV3, request.spreadsheet_token, request.sheet_id
    );

    // 构建查询参数
    let mut api_request: ApiRequest<UpdateSheetResponse> = ApiRequest::put(&url);

    // 添加fields参数
    if !request.fields.is_empty() {
        api_request = api_request.query("fields", &request.fields.join(","));
    }

    // 构建请求体
    let body = json!({
        "properties": request.properties
    });

    api_request = api_request.body(body);

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
    fn test_update_sheet_request_builder() {
        let properties = SheetProperties::new()
            .title("更新的工作表")
            .index(1)
            .grid_properties(GridProperties::new().frozen_row_count(2));

        let request = UpdateSheetRequest::new(
            "spreadsheet_token",
            "sheet_id",
            properties,
            vec!["title".to_string()],
        );

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
        assert_eq!(request.properties.title, Some("更新的工作表".to_string()));
        assert_eq!(request.fields.len(), 1);
    }

    #[test]
    fn test_sheet_properties_builder() {
        let grid_props = GridProperties::new()
            .row_count(1000)
            .column_count(26)
            .frozen_row_count(1)
            .frozen_column_count(1);

        let properties = SheetProperties::new()
            .title("测试工作表")
            .grid_properties(grid_props);

        assert_eq!(properties.title, Some("测试工作表".to_string()));
        assert!(properties.grid_properties.is_some());
        assert_eq!(properties.grid_properties.unwrap().row_count, Some(1000));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateSheetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_sheet_request_with_values() {
        let request = UpdateSheetRequest::new("token", "id", SheetProperties::default(), vec![])
            .spreadsheet_token("new_token")
            .sheet_id("new_id");

        assert_eq!(request.spreadsheet_token, "new_token");
        assert_eq!(request.sheet_id, "new_id");
    }
}