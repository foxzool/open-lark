use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 追加单元格值请求
#[derive(Debug, Serialize, Default)]
pub struct AppendValuesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 追加范围
    pub range: Option<String>,
    /// 值数据
    pub values: Vec<Vec<serde_json::Value>>,
    /// 输入选项
    pub value_input_option: Option<String>,
    /// 插入数据选项
    pub insert_data_option: Option<String>,
}

/// 追加单元格值响应
#[derive(Debug, Deserialize, Default)]
pub struct AppendValuesResponse {
    /// 电子表格ID
    pub spreadsheet_id: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 表格ID
    pub table_id: String,
    /// 更新的范围
    pub updates: AppendUpdates,
    /// 操作结果
    pub result: String,
}

/// 追加更新信息
#[derive(Debug, Deserialize, Default)]
pub struct AppendUpdates {
    /// 更新的范围
    pub updated_range: String,
    /// 更新的行数
    pub updated_rows: i32,
    /// 更新的列数
    pub updated_columns: i32,
    /// 更新的单元格数
    pub updated_cells: i32,
}

/// 追加单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values:append
pub async fn append_values(
    request: AppendValuesRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<AppendValuesResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v2/spreadsheets/{}/values:append",
        config.base_url, request.spreadsheet_token
    );

    let mut query_params = vec![];

    if let Some(range) = &request.range {
        query_params.push(("range".to_string(), range.clone()));
    }

    if let Some(value_input_option) = &request.value_input_option {
        query_params.push(("valueInputOption".to_string(), value_input_option.clone()));
    }

    if let Some(insert_data_option) = &request.insert_data_option {
        query_params.push(("insertDataOption".to_string(), insert_data_option.clone()));
    }

    // 构建请求体
    let mut body_map = serde_json::Map::new();
    body_map.insert("values".to_string(), serde_json::Value::Array(
        request.values.into_iter().map(|v| serde_json::Value::Array(v)).collect()
    ));

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
        headers: vec![],
        query_params,
        body: Some(serde_json::to_vec(&body_map).unwrap()),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_append_values() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = AppendValuesRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            range: Some("A1:B".to_string()),
            values: vec![
                vec![serde_json::Value::String("王五".to_string()), serde_json::Value::Number(serde_json::Number::from(28))],
                vec![serde_json::Value::String("赵六".to_string()), serde_json::Value::Number(serde_json::Number::from(32))],
            ],
            value_input_option: Some("USER_ENTERED".to_string()),
            insert_data_option: Some("INSERT_ROWS".to_string()),
        };

        let result = append_values(request, &config, None).await;
        assert!(result.is_ok());
    }
}