use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 批量获取单元格值请求
#[derive(Debug, Serialize, Default)]
pub struct GetBatchValuesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 范围列表
    pub ranges: Vec<String>,
    /// 值渲染选项
    pub value_render_option: Option<String>,
    /// 日期渲染选项
    pub date_render_option: Option<String>,
}

/// 批量获取单元格值响应
#[derive(Debug, Deserialize, Default)]
pub struct GetBatchValuesResponse {
    /// 值范围列表
    pub value_ranges: Vec<ValueRange>,
    /// 操作结果
    pub result: String,
}

/// 值范围
#[derive(Debug, Deserialize, Default)]
pub struct ValueRange {
    /// 工作表ID
    pub sheet_id: String,
    /// 范围
    pub range: String,
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 结束行
    pub end_row: i32,
    /// 结束列
    pub end_column: i32,
    /// 值数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 批量获取单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values:batchGet
pub async fn get_batch_values(
    request: GetBatchValuesRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetBatchValuesResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v2/spreadsheets/{}/values:batchGet",
        config.base_url, request.spreadsheet_token
    );

    let mut query_params = vec![];

    if !request.ranges.is_empty() {
        query_params.push(("ranges".to_string(), request.ranges.join(",")));
    }

    if let Some(value_render_option) = &request.value_render_option {
        query_params.push(("valueRenderOption".to_string(), value_render_option.clone()));
    }

    if let Some(date_render_option) = &request.date_render_option {
        query_params.push(("dateRenderOption".to_string(), date_render_option.clone()));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params,
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_batch_values() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetBatchValuesRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            ranges: vec![
                "sheet1!A1:B10".to_string(),
                "sheet2!C1:D5".to_string(),
            ],
            value_render_option: Some("DisplayValue".to_string()),
            date_render_option: Some("FormattedString".to_string()),
        };

        let result = get_batch_values(request, &config, None).await;
        assert!(result.is_ok());
    }
}