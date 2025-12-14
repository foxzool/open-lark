use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 删除行列请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteDimensionRangeRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 维度范围信息
    pub dimension_range: DimensionRange,
}

/// 维度范围信息
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct DeleteDimensionRangeResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

/// 电子表格属性
#[derive(Debug, Deserialize, Default)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
    /// 数据区域
    pub data: Option<Vec<GridData>>,
}

/// 工作表属性信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Deserialize, Default)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Deserialize, Default)]
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

/// 删除行列
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dimension_range
pub async fn delete_dimension_range(
    request: DeleteDimensionRangeRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeleteDimensionRangeResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/dimension_range",
        config.base_url, request.spreadsheet_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::DELETE,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_delete_dimension_range() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = DeleteDimensionRangeRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            dimension_range: DimensionRange {
                sheet_id: "test_sheet_id".to_string(),
                dimension: "COLUMNS".to_string(),
                start_index: Some(2),
                end_index: Some(5),
            },
        };

        let result = delete_dimension_range(request, &config, None).await;
        assert!(result.is_ok());
    }
}