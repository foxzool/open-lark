/// 增加行列
///
/// 此接口用于在电子表格中插入新的行或列。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dimension_range
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 增加行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 维度范围信息
    pub dimension_range: DimensionRange,
    /// 插入的行列数
    pub insert_number: i32,
}

impl AddDimensionRangeRequest {
    /// 创建增加行列请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `dimension_range` - 维度范围信息
    /// * `insert_number` - 插入的行列数
    pub fn new(
        spreadsheet_token: impl Into<String>,
        dimension_range: DimensionRange,
        insert_number: i32,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            dimension_range,
            insert_number,
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置维度范围信息
    pub fn dimension_range(mut self, dimension_range: DimensionRange) -> Self {
        self.dimension_range = dimension_range;
        self
    }

    /// 设置插入的行列数
    pub fn insert_number(mut self, insert_number: i32) -> Self {
        self.insert_number = insert_number;
        self
    }
}

/// 维度范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl DimensionRange {
    /// 创建维度范围信息
    ///
    /// # 参数
    /// * `sheet_id` - 工作表ID
    /// * `dimension` - 维度类型：ROWS(行)或COLUMNS(列)
    pub fn new(sheet_id: impl Into<String>, dimension: impl Into<String>) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            dimension: dimension.into(),
            start_index: None,
            end_index: None,
        }
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = sheet_id.into();
        self
    }

    /// 设置维度类型
    pub fn dimension(mut self, dimension: impl Into<String>) -> Self {
        self.dimension = dimension.into();
        self
    }

    /// 设置起始索引
    pub fn start_index(mut self, start_index: i32) -> Self {
        self.start_index = Some(start_index);
        self
    }

    /// 设置结束索引
    pub fn end_index(mut self, end_index: i32) -> Self {
        self.end_index = Some(end_index);
        self
    }
}

/// 增加行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeResponse {
    /// 电子表格属性
    pub data: Option<SpreadsheetProperties>,
}

impl ApiResponseTrait for AddDimensionRangeResponse {
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

/// 增加行列
///
/// 在指定电子表格中插入新的行或列。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dimension_range
pub async fn add_dimension_range(
    request: AddDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<AddDimensionRangeResponse>> {
    // 使用SheetsApiV3枚举生成API端点
    let api_endpoint = SheetsApiV3::DimensionRange(request.spreadsheet_token);

    // 创建API请求
    let mut api_request: ApiRequest<AddDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(request);

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
    fn test_add_dimension_range_request_builder() {
        let dimension_range = DimensionRange::new("sheet_id", "ROWS")
            .start_index(5)
            .end_index(6);
        let request = AddDimensionRangeRequest::new("spreadsheet_token", dimension_range, 10);

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.insert_number, 10);
    }

    #[test]
    fn test_dimension_range_builder() {
        let dimension_range = DimensionRange::new("sheet_id", "ROWS")
            .start_index(5)
            .end_index(6);

        assert_eq!(dimension_range.sheet_id, "sheet_id");
        assert_eq!(dimension_range.dimension, "ROWS");
        assert_eq!(dimension_range.start_index, Some(5));
        assert_eq!(dimension_range.end_index, Some(6));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(AddDimensionRangeResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_api_endpoint() {
        let endpoint = SheetsApiV3::DimensionRange("test_token".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/sheets/v3/spreadsheets/test_token/dimension_range");
    }
}