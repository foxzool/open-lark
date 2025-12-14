use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 批量更新工作表请求
#[derive(Debug, Serialize, Default)]
pub struct BatchUpdateSheetsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 批量更新请求列表
    pub requests: Vec<SheetUpdateRequest>,
    /// 是否包含电子表格响应
    pub include_spreadsheet_response: Option<bool>,
}

/// 工作表更新请求
#[derive(Debug, Serialize, Default)]
#[serde(tag = "updateSheetProperties")]
pub struct SheetUpdateRequest {
    /// 工作表属性
    pub properties: SheetProperties,
    /// 更新字段列表
    pub fields: Vec<String>,
}

/// 工作表属性
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct BatchUpdateSheetsResponse {
    /// 电子表格属性
    pub spreadsheet: Option<SpreadsheetProperties>,
    /// 更新结果列表
    pub replies: Vec<SheetUpdateResponse>,
    /// 操作结果
    pub result: String,
}

/// 工作表更新响应
#[derive(Debug, Deserialize, Default)]
pub struct SheetUpdateResponse {
    /// 工作表属性
    pub properties: Option<SheetPropertiesInfo>,
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

/// 批量更新工作表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets:batchUpdate
pub async fn batch_update_sheets(
    request: BatchUpdateSheetsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<BatchUpdateSheetsResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets:batchUpdate",
        config.base_url, request.spreadsheet_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
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
    async fn test_batch_update_sheets() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = BatchUpdateSheetsRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            requests: vec![
                SheetUpdateRequest {
                    properties: SheetProperties {
                        sheet_id: "sheet1".to_string(),
                        title: Some("更新的工作表1".to_string()),
                        index: Some(0),
                        sheet_type: None,
                        grid_properties: None,
                    },
                    fields: vec!["title".to_string()],
                },
                SheetUpdateRequest {
                    properties: SheetProperties {
                        sheet_id: "sheet2".to_string(),
                        title: Some("更新的工作表2".to_string()),
                        index: Some(1),
                        sheet_type: None,
                        grid_properties: Some(GridProperties {
                            frozen_row_count: Some(1),
                            frozen_column_count: None,
                            row_count: None,
                            column_count: None,
                        }),
                    },
                    fields: vec!["title".to_string(), "grid_properties.frozen_row_count".to_string()],
                },
            ],
            include_spreadsheet_response: Some(true),
        };

        let result = batch_update_sheets(request, &config, None).await;
        assert!(result.is_ok());
    }
}