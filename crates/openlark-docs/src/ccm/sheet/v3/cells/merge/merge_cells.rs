use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 合并单元格请求
#[derive(Debug, Serialize, Default)]
pub struct MergeCellsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 合并类型
    pub merge_type: String,
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 结束行
    pub end_row: i32,
    /// 结束列
    pub end_column: i32,
}

/// 合并单元格响应
#[derive(Debug, Deserialize, Default)]
pub struct MergeCellsResponse {
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
}

/// 合并单元格
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/merge
pub async fn merge_cells(
    request: MergeCellsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<MergeCellsResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/merge",
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
    async fn test_merge_cells() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = MergeCellsRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            merge_type: "MERGE_ALL".to_string(),
            start_row: 0,
            start_column: 0,
            end_row: 1,
            end_column: 2,
        };

        let result = merge_cells(request, &config, None).await;
        assert!(result.is_ok());
    }
}