use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 追加单元格值请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
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
#[derive(Debug, Clone, Deserialize, Default)]
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

impl ApiResponseTrait for AppendValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppendValuesRequest {
    /// 创建新的追加单元格值请求构建器
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

    /// 设置追加范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
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

    /// 设置插入数据选项
    pub fn insert_data_option(mut self, option: impl Into<String>) -> Self {
        self.insert_data_option = Some(option.into());
        self
    }
}

/// 追加单元格值
/// docPath: https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values:append
pub async fn append_values(
    request: AppendValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<AppendValuesResponse>> {
    // 构建查询参数
    let mut query_params = Vec::new();

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
    let body = json!(body_map);

    // 创建API请求
    let mut api_request: ApiRequest<AppendValuesResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/values:append", SheetsApiV3, request.spreadsheet_token))
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
    fn test_append_values_request_builder() {
        let values = vec![
            vec![serde_json::Value::String("王五".to_string()), serde_json::Value::Number(serde_json::Number::from(28))],
            vec![serde_json::Value::String("赵六".to_string()), serde_json::Value::Number(serde_json::Number::from(32))],
        ];

        let request = AppendValuesRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .range("A1:B")
            .values(values.clone())
            .value_input_option("USER_ENTERED")
            .insert_data_option("INSERT_ROWS");

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.range, Some("A1:B".to_string()));
        assert_eq!(request.values, values);
        assert_eq!(request.value_input_option, Some("USER_ENTERED".to_string()));
        assert_eq!(request.insert_data_option, Some("INSERT_ROWS".to_string()));
    }
}