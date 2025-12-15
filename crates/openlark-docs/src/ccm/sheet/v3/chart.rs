use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 创建图表请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateChartRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 图表信息
    pub chart: ChartInfo,
}

/// 图表信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateChartResponse {
    /// 图表ID
    pub chart_id: String,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for CreateChartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateChartRequest {
    /// 创建新的创建图表请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置图表信息
    pub fn chart(mut self, chart: ChartInfo) -> Self {
        self.chart = chart;
        self
    }
}

impl ChartInfo {
    /// 创建新的图表信息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置图表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// 设置图表类型
    pub fn chart_type(mut self, chart_type: impl Into<String>) -> Self {
        self.chart_type = chart_type.into();
        self
    }

    /// 设置数据范围
    pub fn data_range(mut self, range: impl Into<String>) -> Self {
        self.data_range = range.into();
        self
    }

    /// 设置X轴标题
    pub fn x_axis_title(mut self, title: impl Into<String>) -> Self {
        self.x_axis_title = Some(title.into());
        self
    }

    /// 设置Y轴标题
    pub fn y_axis_title(mut self, title: impl Into<String>) -> Self {
        self.y_axis_title = Some(title.into());
        self
    }
}

/// 创建图表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/charts
pub async fn create_chart(
    request: CreateChartRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateChartResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<CreateChartResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/charts", SheetsApiV3, request.spreadsheet_token))
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
    fn test_create_chart_request_builder() {
        let request = CreateChartRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .chart(
                ChartInfo::new()
                    .title("销售统计图")
                    .chart_type("COLUMN")
                    .data_range("A1:B10")
                    .x_axis_title("月份")
                    .y_axis_title("销售额")
            );

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.chart.title, "销售统计图");
        assert_eq!(request.chart.chart_type, "COLUMN");
        assert_eq!(request.chart.data_range, "A1:B10");
        assert_eq!(request.chart.x_axis_title, Some("月份".to_string()));
        assert_eq!(request.chart.y_axis_title, Some("销售额".to_string()));
    }
}