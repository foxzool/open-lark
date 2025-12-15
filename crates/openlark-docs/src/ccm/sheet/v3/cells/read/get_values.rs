use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::SheetsApiV3;

/// 获取单元格值请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 起始行
    pub start_row: Option<i32>,
    /// 起始列
    pub start_column: Option<i32>,
    /// 结束行
    pub end_row: Option<i32>,
    /// 结束列
    pub end_column: Option<i32>,
    /// 值渲染选项
    pub value_render_option: Option<String>,
    /// 日期渲染选项
    pub date_render_option: Option<String>,
}

impl GetValuesRequest {
    /// 创建获取单元格值请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            start_row: None,
            start_column: None,
            end_row: None,
            end_column: None,
            value_render_option: None,
            date_render_option: None,
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = sheet_id.into();
        self
    }

    /// 设置起始行
    pub fn start_row(mut self, start_row: i32) -> Self {
        self.start_row = Some(start_row);
        self
    }

    /// 设置起始列
    pub fn start_column(mut self, start_column: i32) -> Self {
        self.start_column = Some(start_column);
        self
    }

    /// 设置结束行
    pub fn end_row(mut self, end_row: i32) -> Self {
        self.end_row = Some(end_row);
        self
    }

    /// 设置结束列
    pub fn end_column(mut self, end_column: i32) -> Self {
        self.end_column = Some(end_column);
        self
    }

    /// 设置值渲染选项
    pub fn value_render_option(mut self, value_render_option: impl Into<String>) -> Self {
        self.value_render_option = Some(value_render_option.into());
        self
    }

    /// 设置日期渲染选项
    pub fn date_render_option(mut self, date_render_option: impl Into<String>) -> Self {
        self.date_render_option = Some(date_render_option.into());
        self
    }

    /// 将列号转换为字母表示
    fn number_to_column(n: i32) -> String {
        let mut result = String::new();
        let mut num = n;

        while num > 0 {
            num -= 1;
            result.insert(0, char::from((num % 26 + 65) as u8));
            num /= 26;
        }

        result
    }
}

/// 获取单元格值响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuesResponse {
    /// 值范围
    pub value_range: ValueRange,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for GetValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 值范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueRange {
    /// 工作表ID
    pub sheet_id: String,
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

/// 获取单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:sheetId!A1:B10
pub async fn get_values(
    request: GetValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetValuesResponse>> {
    // 构建范围表达式
    let mut range_parts = vec![request.sheet_id.clone()];

    // 构建A1表示法范围
    if let (Some(sr), Some(sc)) = (request.start_row, request.start_column) {
        let start_cell = format!("{}{}",
            GetValuesRequest::number_to_column(sc + 1),
            sr + 1
        );
        range_parts.push(start_cell);

        if let (Some(er), Some(ec)) = (request.end_row, request.end_column) {
            let end_cell = format!("{}{}",
                GetValuesRequest::number_to_column(ec + 1),
                er + 1
            );
            range_parts.push(end_cell);
        }
    }

    let range = range_parts.join("!");

    // 使用 sheets v2 端点
    let url = format!(
        "/open-apis/sheets/v2/spreadsheets/{}/values/{}",
        request.spreadsheet_token, range
    );

    // 创建API请求
    let mut api_request: ApiRequest<GetValuesResponse> = ApiRequest::get(&url);

    // 添加查询参数
    if let Some(value_render_option) = &request.value_render_option {
        api_request = api_request.query("valueRenderOption", value_render_option);
    }

    if let Some(date_render_option) = &request.date_render_option {
        api_request = api_request.query("dateRenderOption", date_render_option);
    }

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
    fn test_get_values_request_builder() {
        let request = GetValuesRequest::new("spreadsheet_token", "sheet_id")
            .start_row(0)
            .start_column(0)
            .end_row(9)
            .end_column(1);

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
        assert_eq!(request.start_row, Some(0));
        assert_eq!(request.start_column, Some(0));
    }

    #[test]
    fn test_get_values_request_with_options() {
        let request = GetValuesRequest::new("token", "id")
            .value_render_option("DisplayValue")
            .date_render_option("FormattedString");

        assert_eq!(request.value_render_option, Some("DisplayValue".to_string()));
        assert_eq!(request.date_render_option, Some("FormattedString".to_string()));
    }

    #[test]
    fn test_number_to_column() {
        assert_eq!(GetValuesRequest::number_to_column(1), "A");
        assert_eq!(GetValuesRequest::number_to_column(26), "Z");
        assert_eq!(GetValuesRequest::number_to_column(27), "AA");
        assert_eq!(GetValuesRequest::number_to_column(52), "AZ");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetValuesResponse::data_format(), ResponseFormat::Data);
    }
}