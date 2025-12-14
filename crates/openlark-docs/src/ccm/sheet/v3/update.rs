use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 更新工作表属性请求
#[derive(Debug, Serialize, Default)]
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

/// 工作表属性
#[derive(Debug, Serialize, Default)]
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

/// 网格属性
#[derive(Debug, Serialize, Default)]
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

/// 更新工作表属性响应
#[derive(Debug, Deserialize, Default)]
pub struct UpdateSheetResponse {
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

/// 更新工作表属性
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId
pub async fn update_sheet(
    request: UpdateSheetRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<UpdateSheetResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}",
        config.base_url, request.spreadsheet_token, request.sheet_id
    );

    let mut query_params = vec![];

    if !request.fields.is_empty() {
        query_params.push(("fields".to_string(), request.fields.join(",")));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::PUT,
        headers: vec![],
        query_params,
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_update_sheet() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = UpdateSheetRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            properties: SheetProperties {
                title: Some("更新的工作表".to_string()),
                index: Some(1),
                sheet_type: None,
                grid_properties: Some(GridProperties {
                    row_count: Some(2000),
                    column_count: None,
                    frozen_row_count: Some(2),
                    frozen_column_count: Some(1),
                }),
            },
            fields: vec!["title".to_string(), "grid_properties.frozen_row_count".to_string()],
        };

        let result = update_sheet(request, &config, None).await;
        assert!(result.is_ok());
    }
}