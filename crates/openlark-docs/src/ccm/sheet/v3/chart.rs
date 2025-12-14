use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建图表请求
#[derive(Debug, Serialize, Default)]
pub struct CreateChartRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 图表信息
    pub chart: ChartInfo,
}

/// 图表信息
#[derive(Debug, Serialize, Default)]
pub struct ChartInfo {
    /// 图表标题
    pub title: String,
    /// 图表类型
    pub chart_type: String,
    /// 数据范围
    pub data_range: String,
    /// X轴标题
    pub x_axis_title: Option<String>,
    /// Y轴标题
    pub y_axis_title: Option<String>,
}

/// 创建图表响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateChartResponse {
    /// 图表ID
    pub chart_id: String,
    /// 操作结果
    pub result: String,
}

/// 创建图表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/charts
pub async fn create_chart(
    request: CreateChartRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateChartResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/charts",
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
    async fn test_create_chart() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateChartRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            chart: ChartInfo {
                title: "销售统计图".to_string(),
                chart_type: "COLUMN".to_string(),
                data_range: "A1:B10".to_string(),
                x_axis_title: Some("月份".to_string()),
                y_axis_title: Some("销售额".to_string()),
            },
        };

        let result = create_chart(request, &config, None).await;
        assert!(result.is_ok());
    }
}
