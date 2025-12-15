use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 写入单元格值请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WriteValuesRequest {
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
    /// 值数据
    pub values: Vec<Vec<serde_json::Value>>,
    /// 输入选项
    pub value_input_option: Option<String>,
}

/// 写入单元格值响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct WriteValuesResponse {
    /// 更新的值范围
    pub updated_range: String,
    /// 更新的行数
    pub updated_rows: i32,
    /// 更新的列数
    pub updated_columns: i32,
    /// 更新的单元格数
    pub updated_cells: i32,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for WriteValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl WriteValuesRequest {
    /// 创建新的写入单元格值请求构建器
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

    /// 设置起始行
    pub fn start_row(mut self, row: i32) -> Self {
        self.start_row = Some(row);
        self
    }

    /// 设置起始列
    pub fn start_column(mut self, column: i32) -> Self {
        self.start_column = Some(column);
        self
    }

    /// 设置结束行
    pub fn end_row(mut self, row: i32) -> Self {
        self.end_row = Some(row);
        self
    }

    /// 设置结束列
    pub fn end_column(mut self, column: i32) -> Self {
        self.end_column = Some(column);
        self
    }

    /// 设置值数据
    pub fn values(mut self, values: Vec<Vec<serde_json::Value>>) -> Self {
        self.values = values;
        self
    }

    /// 设置输入选项
    pub fn value_input_option(mut self, option: impl Into<String>) -> Self {
        self.value_input_option = Some(option.into());
        self
    }

    /// 构建范围表达式
    fn build_range(&self) -> String {
        let mut range_parts = vec![self.sheet_id.clone()];

        // 构建A1表示法范围
        if let (Some(sr), Some(sc)) = (self.start_row, self.start_column) {
            let start_cell = format!("{}{}", Self::number_to_column(sc + 1), sr + 1);
            range_parts.push(start_cell);

            if let (Some(er), Some(ec)) = (self.end_row, self.end_column) {
                let end_cell = format!("{}{}", Self::number_to_column(ec + 1), er + 1);
                range_parts.push(end_cell);
            }
        }

        range_parts.join("!")
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

/// 写入单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values
pub async fn write_values(
    request: WriteValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<WriteValuesResponse>> {
    // 构建范围表达式
    let range = request.build_range();

    // 构建查询参数
    let mut query_params = Vec::new();
    if let Some(value_input_option) = &request.value_input_option {
        query_params.push(("valueInputOption".to_string(), value_input_option.clone()));
    }

    // 构建请求体
    let mut body_map = serde_json::Map::new();
    body_map.insert("values".to_string(), serde_json::Value::Array(
        request.values.into_iter().map(|v| serde_json::Value::Array(v)).collect()
    ));
    let body = json!(body_map);

    // 创建API请求
    let mut api_request: ApiRequest<WriteValuesResponse> =
        ApiRequest::put(&format!("{}/spreadsheets/{}/values/{}", SheetsApiV3, request.spreadsheet_token, range))
            .query_params(query_params)
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
    fn test_write_values_request_builder() {
        let values = vec![
            vec![serde_json::Value::String("姓名".to_string()), serde_json::Value::String("年龄".to_string())],
            vec![serde_json::Value::String("张三".to_string()), serde_json::Value::Number(serde_json::Number::from(25))],
            vec![serde_json::Value::String("李四".to_string()), serde_json::Value::Number(serde_json::Number::from(30))],
        ];

        let request = WriteValuesRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .start_row(0)
            .start_column(0)
            .end_row(2)
            .end_column(1)
            .values(values.clone())
            .value_input_option("USER_ENTERED");

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.start_row, Some(0));
        assert_eq!(request.start_column, Some(0));
        assert_eq!(request.end_row, Some(2));
        assert_eq!(request.end_column, Some(1));
        assert_eq!(request.values, values);
        assert_eq!(request.value_input_option, Some("USER_ENTERED".to_string()));
    }

    #[test]
    fn test_number_to_column() {
        assert_eq!(WriteValuesRequest::number_to_column(1), "A");
        assert_eq!(WriteValuesRequest::number_to_column(26), "Z");
        assert_eq!(WriteValuesRequest::number_to_column(27), "AA");
        assert_eq!(WriteValuesRequest::number_to_column(52), "AZ");
        assert_eq!(WriteValuesRequest::number_to_column(53), "BA");
    }

    #[test]
    fn test_build_range() {
        let request = WriteValuesRequest {
            sheet_id: "Sheet1".to_string(),
            start_row: Some(0),
            start_column: Some(0),
            end_row: Some(2),
            end_column: Some(1),
            ..Default::default()
        };

        assert_eq!(request.build_range(), "Sheet1!A1:C3");

        let request2 = WriteValuesRequest {
            sheet_id: "Sheet1".to_string(),
            start_row: Some(0),
            start_column: Some(0),
            end_row: None,
            end_column: None,
            ..Default::default()
        };

        assert_eq!(request2.build_range(), "Sheet1!A1");
    }
}