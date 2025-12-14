use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 设置筛选请求
#[derive(Debug, Serialize, Default)]
pub struct SetFilterRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 筛选范围
    pub range: String,
    /// 筛选条件
    pub criteria: Vec<FilterCriteria>,
}

/// 筛选条件
#[derive(Debug, Serialize, Default)]
pub struct FilterCriteria {
    /// 列索引
    pub column_index: i32,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub values: Vec<String>,
    /// 条件
    pub condition: Option<FilterCondition>,
}

/// 筛选条件
#[derive(Debug, Serialize, Default)]
pub struct FilterCondition {
    /// 条件类型
    pub r#type: String,
    /// 值
    pub value: String,
}

/// 设置筛选响应
#[derive(Debug, Deserialize, Default)]
pub struct SetFilterResponse {
    /// 筛选信息
    pub filter: FilterInfo,
    /// 操作结果
    pub result: String,
}

/// 筛选信息
#[derive(Debug, Deserialize, Default)]
pub struct FilterInfo {
    /// 筛选ID
    pub filter_id: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 筛选范围
    pub range: String,
    /// 筛选条件
    pub criteria: Vec<FilterCriteriaInfo>,
}

/// 筛选条件信息
#[derive(Debug, Deserialize, Default)]
pub struct FilterCriteriaInfo {
    /// 列索引
    pub column_index: i32,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub values: Vec<String>,
}

/// 设置筛选
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/filter
pub async fn set_filter(
    request: SetFilterRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SetFilterResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/filter",
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
    async fn test_set_filter() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = SetFilterRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            range: "A1:D10".to_string(),
            criteria: vec![
                FilterCriteria {
                    column_index: 0,
                    filter_type: "TEXT_CONTAINS".to_string(),
                    values: vec!["测试".to_string()],
                    condition: None,
                },
                FilterCriteria {
                    column_index: 1,
                    filter_type: "NUMBER_GREATER_THAN".to_string(),
                    values: vec![],
                    condition: Some(FilterCondition {
                        r#type: "NUMBER_GREATER_THAN".to_string(),
                        value: "100".to_string(),
                    }),
                },
            ],
        };

        let result = set_filter(request, &config, None).await;
        assert!(result.is_ok());
    }
}