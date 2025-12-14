use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 写入单元格值请求
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
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

/// 写入单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values
pub async fn write_values(
    request: WriteValuesRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<WriteValuesResponse>> {
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

    if let Some(value_input_option) = &request.value_input_option {
        query_params.push(("valueInputOption".to_string(), value_input_option.clone()));
    }

    // 构建请求体
    let mut body_map = serde_json::Map::new();
    body_map.insert("values".to_string(), serde_json::Value::Array(
        request.values.into_iter().map(|v| serde_json::Value::Array(v)).collect()
    ));

    let req = OpenLarkRequest {
        url,
        method: http::Method::PUT,
        headers: vec![],
        query_params,
        body: Some(serde_json::to_vec(&body_map).unwrap()),
    };

    OpenLarkClient::request(req, config, option).await
}

impl WriteValuesRequest {
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
    async fn test_write_values() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = WriteValuesRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            start_row: Some(0),
            start_column: Some(0),
            end_row: Some(2),
            end_column: Some(1),
            values: vec![
                vec![serde_json::Value::String("姓名".to_string()), serde_json::Value::String("年龄".to_string())],
                vec![serde_json::Value::String("张三".to_string()), serde_json::Value::Number(serde_json::Number::from(25))],
                vec![serde_json::Value::String("李四".to_string()), serde_json::Value::Number(serde_json::Number::from(30))],
            ],
            value_input_option: Some("USER_ENTERED".to_string()),
        };

        let result = write_values(request, &config, None).await;
        assert!(result.is_ok());
    }
}