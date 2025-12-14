use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 重新计算请求
#[derive(Debug, Serialize, Default)]
pub struct RecalculateRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID（可选，不指定则计算全部）
    pub sheet_id: Option<String>,
    /// 计算范围（可选）
    pub range: Option<String>,
}

/// 重新计算响应
#[derive(Debug, Deserialize, Default)]
pub struct RecalculateResponse {
    /// 计算ID
    pub calculation_id: String,
    /// 操作结果
    pub result: String,
}

/// 重新计算
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/recalculate
pub async fn recalculate(
    request: RecalculateRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<RecalculateResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/recalculate",
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
    async fn test_recalculate() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = RecalculateRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: Some("test_sheet_id".to_string()),
            range: Some("A1:D10".to_string()),
        };

        let result = recalculate(request, &config, None).await;
        assert!(result.is_ok());
    }
}