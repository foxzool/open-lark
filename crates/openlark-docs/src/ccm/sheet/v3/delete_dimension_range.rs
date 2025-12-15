use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除行列请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteDimensionRangeRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 维度范围信息
    pub dimension_range: DimensionRange,
}

/// 维度范围信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DimensionRange {
    /// 工作表ID
    pub sheet_id: String,
    /// 维度类型：ROWS(行)或COLUMNS(列)
    pub dimension: String,
    /// 起始索引
    pub start_index: Option<i32>,
    /// 结束索引
    pub end_index: Option<i32>,
}

/// 删除行列响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteDimensionRangeResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

/// 电子表格属性
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
    /// 数据区域
    pub data: Option<Vec<GridData>>,
}

/// 工作表属性信息
#[derive(Debug, Clone, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Clone, Deserialize, Default)]
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

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteDimensionRangeRequest {
    /// 创建新的删除行列请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置维度范围信息
    pub fn dimension_range(mut self, range: DimensionRange) -> Self {
        self.dimension_range = range;
        self
    }
}

impl DimensionRange {
    /// 创建新的维度范围构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置维度类型
    pub fn dimension(mut self, dimension: impl Into<String>) -> Self {
        self.dimension = dimension.into();
        self
    }

    /// 设置起始索引
    pub fn start_index(mut self, index: i32) -> Self {
        self.start_index = Some(index);
        self
    }

    /// 设置结束索引
    pub fn end_index(mut self, index: i32) -> Self {
        self.end_index = Some(index);
        self
    }

    /// 设置索引范围
    pub fn index_range(mut self, start: i32, end: i32) -> Self {
        self.start_index = Some(start);
        self.end_index = Some(end);
        self
    }

    /// 创建行维度
    pub fn rows(sheet_id: impl Into<String>) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            dimension: "ROWS".to_string(),
            start_index: None,
            end_index: None,
        }
    }

    /// 创建列维度
    pub fn columns(sheet_id: impl Into<String>) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            dimension: "COLUMNS".to_string(),
            start_index: None,
            end_index: None,
        }
    }

    /// 创建带范围的行维度
    pub fn rows_with_range(sheet_id: impl Into<String>, start: i32, end: i32) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            dimension: "ROWS".to_string(),
            start_index: Some(start),
            end_index: Some(end),
        }
    }

    /// 创建带范围的列维度
    pub fn columns_with_range(sheet_id: impl Into<String>, start: i32, end: i32) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            dimension: "COLUMNS".to_string(),
            start_index: Some(start),
            end_index: Some(end),
        }
    }
}

/// 删除行列
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dimension_range
pub async fn delete_dimension_range(
    request: DeleteDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteDimensionRangeResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<DeleteDimensionRangeResponse> =
        ApiRequest::delete(&format!("{}/spreadsheets/{}/dimension_range", SheetsApiV3, request.spreadsheet_token))
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
    fn test_delete_dimension_range_request_builder() {
        let dimension_range = DimensionRange::columns("test_sheet")
            .index_range(2, 5);

        let request = DeleteDimensionRangeRequest::new()
            .spreadsheet_token("test_token")
            .dimension_range(dimension_range);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.dimension_range.sheet_id, "test_sheet");
        assert_eq!(request.dimension_range.dimension, "COLUMNS");
        assert_eq!(request.dimension_range.start_index, Some(2));
        assert_eq!(request.dimension_range.end_index, Some(5));
    }

    #[test]
    fn test_dimension_range_convenience_methods() {
        let rows = DimensionRange::rows("sheet1");
        assert_eq!(rows.sheet_id, "sheet1");
        assert_eq!(rows.dimension, "ROWS");
        assert_eq!(rows.start_index, None);
        assert_eq!(rows.end_index, None);

        let columns = DimensionRange::columns("sheet2");
        assert_eq!(columns.sheet_id, "sheet2");
        assert_eq!(columns.dimension, "COLUMNS");
        assert_eq!(columns.start_index, None);
        assert_eq!(columns.end_index, None);

        let rows_with_range = DimensionRange::rows_with_range("sheet1", 0, 10);
        assert_eq!(rows_with_range.sheet_id, "sheet1");
        assert_eq!(rows_with_range.dimension, "ROWS");
        assert_eq!(rows_with_range.start_index, Some(0));
        assert_eq!(rows_with_range.end_index, Some(10));

        let columns_with_range = DimensionRange::columns_with_range("sheet2", 5, 15);
        assert_eq!(columns_with_range.sheet_id, "sheet2");
        assert_eq!(columns_with_range.dimension, "COLUMNS");
        assert_eq!(columns_with_range.start_index, Some(5));
        assert_eq!(columns_with_range.end_index, Some(15));
    }

    #[test]
    fn test_dimension_range_builder() {
        let range = DimensionRange::new()
            .sheet_id("test_sheet")
            .dimension("COLUMNS")
            .index_range(2, 5);

        assert_eq!(range.sheet_id, "test_sheet");
        assert_eq!(range.dimension, "COLUMNS");
        assert_eq!(range.start_index, Some(2));
        assert_eq!(range.end_index, Some(5));
    }
}