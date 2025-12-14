use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 查找替换请求
#[derive(Debug, Serialize, Default)]
pub struct FindReplaceRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 查找内容
    pub find: String,
    /// 替换内容
    pub replacement: String,
    /// 查找范围
    pub range: Option<String>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
    /// 是否全词匹配
    pub match_entire_cell: Option<bool>,
    /// 是否搜索公式
    pub search_formulas: Option<bool>,
    /// 是否包含正则表达式
    pub include_regex: Option<bool>,
}

/// 查找替换响应
#[derive(Debug, Deserialize, Default)]
pub struct FindReplaceResponse {
    /// 查找替换结果
    pub find_replace_response: FindReplaceResult,
    /// 操作结果
    pub result: String,
}

/// 查找替换结果
#[derive(Debug, Deserialize, Default)]
pub struct FindReplaceResult {
    /// 找到的单元格数
    pub num_found: i32,
    /// 替换的单元格数
    pub num_changed: i32,
    /// 公式修改的单元格数
    pub formulas_changed: i32,
    /// 查找的单元格列表
    pub values: Vec<FoundValue>,
}

/// 找到的值
#[derive(Debug, Deserialize, Default)]
pub struct FoundValue {
    /// 工作表ID
    pub sheet_id: String,
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 找到的值
    pub value: serde_json::Value,
}

/// 查找替换
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/findReplace
pub async fn find_replace(
    request: FindReplaceRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<FindReplaceResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/findReplace",
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
    async fn test_find_replace() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = FindReplaceRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            find: "旧值".to_string(),
            replacement: "新值".to_string(),
            range: Some("A1:C10".to_string()),
            case_sensitive: Some(false),
            match_entire_cell: Some(false),
            search_formulas: Some(false),
            include_regex: Some(false),
        };

        let result = find_replace(request, &config, None).await;
        assert!(result.is_ok());
    }
}