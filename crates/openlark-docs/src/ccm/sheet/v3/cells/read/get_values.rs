use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取单元格值请求
#[derive(Debug, Serialize, Default)]
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

/// 获取单元格值响应
#[derive(Debug, Deserialize, Default)]
pub struct GetValuesResponse {
    /// 值范围
    pub value_range: ValueRange,
    /// 操作结果
    pub result: String,
}

/// 值范围
#[derive(Debug, Deserialize, Default)]
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
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetValuesResponse>> {
    // 构建范围表达式
    let mut range_parts = vec![request.sheet_id.clone()];

    // 构建A1表示法范围
    if let (Some(sr), Some(sc)) = (request.start_row, request.start_column) {
        let start_cell = format!("{}{}",
            Self::number_to_column(sc + 1),
            sr + 1
        );
        range_parts.push(start_cell);

        if let (Some(er), Some(ec)) = (request.end_row, request.end_column) {
            let end_cell = format!("{}{}",
                Self::number_to_column(ec + 1),
                er + 1
            );
            range_parts.push(end_cell);
        }
    }

    let range = range_parts.join("!");

    let url = format!(
        "{}/open-apis/sheets/v2/spreadsheets/{}/values/{}",
        config.base_url, request.spreadsheet_token, range
    );

    let mut query_params = vec![];

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

impl GetValuesRequest {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_values() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetValuesRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            start_row: Some(0),
            start_column: Some(0),
            end_row: Some(9),
            end_column: Some(1),
            value_render_option: Some("DisplayValue".to_string()),
            date_render_option: Some("FormattedString".to_string()),
        };

        let result = get_values(request, &config, None).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_number_to_column() {
        assert_eq!(GetValuesRequest::number_to_column(1), "A");
        assert_eq!(GetValuesRequest::number_to_column(26), "Z");
        assert_eq!(GetValuesRequest::number_to_column(27), "AA");
        assert_eq!(GetValuesRequest::number_to_column(52), "AZ");
    }
}