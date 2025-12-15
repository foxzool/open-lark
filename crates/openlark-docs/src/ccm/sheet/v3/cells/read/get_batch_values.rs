use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 批量获取单元格值请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetBatchValuesResponse {
    /// 值范围列表
    pub value_ranges: Vec<ValueRange>,
    /// 操作结果
    pub result: String,
}

/// 值范围
#[derive(Debug, Clone, Deserialize, Default)]
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

impl ApiResponseTrait for GetBatchValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetBatchValuesRequest {
    /// 创建新的批量获取单元格值请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 添加范围
    pub fn add_range(mut self, range: impl Into<String>) -> Self {
        self.ranges.push(range.into());
        self
    }

    /// 设置范围列表
    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        self.ranges = ranges;
        self
    }

    /// 设置值渲染选项
    pub fn value_render_option(mut self, option: impl Into<String>) -> Self {
        self.value_render_option = Some(option.into());
        self
    }

    /// 设置日期渲染选项
    pub fn date_render_option(mut self, option: impl Into<String>) -> Self {
        self.date_render_option = Some(option.into());
        self
    }
}

/// 批量获取单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values:batchGet
pub async fn get_batch_values(
    request: GetBatchValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetBatchValuesResponse>> {
    // 构建查询参数
    let mut query_params = Vec::new();

    if !request.ranges.is_empty() {
        query_params.push(("ranges".to_string(), request.ranges.join(",")));
    }

    if let Some(value_render_option) = &request.value_render_option {
        query_params.push(("valueRenderOption".to_string(), value_render_option.clone()));
    }

    if let Some(date_render_option) = &request.date_render_option {
        query_params.push(("dateRenderOption".to_string(), date_render_option.clone()));
    }

    // 创建API请求
    let mut api_request: ApiRequest<GetBatchValuesResponse> =
        ApiRequest::get(&format!("{}/spreadsheets/{}/values:batchGet", SheetsApiV3, request.spreadsheet_token))
            .query_params(query_params);

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
    fn test_get_batch_values_request_builder() {
        let request = GetBatchValuesRequest::new()
            .spreadsheet_token("test_token")
            .add_range("sheet1!A1:B10")
            .add_range("sheet2!C1:D5")
            .value_render_option("DisplayValue")
            .date_render_option("FormattedString");

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.ranges, vec!["sheet1!A1:B10", "sheet2!C1:D5"]);
        assert_eq!(request.value_render_option, Some("DisplayValue".to_string()));
        assert_eq!(request.date_render_option, Some("FormattedString".to_string()));
    }

    #[test]
    fn test_get_batch_values_request_with_ranges() {
        let ranges = vec![
            "sheet1!A1:B10".to_string(),
            "sheet2!C1:D5".to_string(),
        ];

        let request = GetBatchValuesRequest::new()
            .spreadsheet_token("test_token")
            .ranges(ranges.clone());

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.ranges, ranges);
    }
}