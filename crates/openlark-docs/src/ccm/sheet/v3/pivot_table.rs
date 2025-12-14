use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建数据透视表请求
#[derive(Debug, Serialize, Default)]
pub struct CreatePivotTableRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 源工作表ID
    pub source_sheet_id: String,
    /// 数据范围
    pub source_range: String,
    /// 目标工作表ID
    pub target_sheet_id: String,
    /// 行字段
    pub row_fields: Vec<String>,
    /// 列字段
    pub column_fields: Vec<String>,
    /// 值字段
    pub value_fields: Vec<ValueField>,
}

/// 值字段
#[derive(Debug, Serialize, Default)]
pub struct ValueField {
    /// 字段名称
    pub field_name: String,
    /// 汇总函数
    pub summarize_function: String,
}

/// 创建数据透视表响应
#[derive(Debug, Deserialize, Default)]
pub struct CreatePivotTableResponse {
    /// 透视表ID
    pub pivot_table_id: String,
    /// 操作结果
    pub result: String,
}

/// 创建数据透视表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/pivotTables
pub async fn create_pivot_table(
    request: CreatePivotTableRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreatePivotTableResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/pivotTables",
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
    async fn test_create_pivot_table() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreatePivotTableRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            source_sheet_id: "source_sheet".to_string(),
            source_range: "A1:D100".to_string(),
            target_sheet_id: "target_sheet".to_string(),
            row_fields: vec!["category".to_string()],
            column_fields: vec!["month".to_string()],
            value_fields: vec![
                ValueField {
                    field_name: "amount".to_string(),
                    summarize_function: "SUM".to_string(),
                },
            ],
        };

        let result = create_pivot_table(request, &config, None).await;
        assert!(result.is_ok());
    }
}
