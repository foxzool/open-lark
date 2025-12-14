use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 复制工作表请求
#[derive(Debug, Serialize, Default)]
pub struct CopyToRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 源工作表ID
    pub sheet_id: String,
    /// 目标电子表格token
    pub destination_spreadsheet_token: String,
}

/// 复制工作表响应
#[derive(Debug, Deserialize, Default)]
pub struct CopyToResponse {
    /// 新工作表ID
    pub sheet_id: String,
    /// 操作结果
    pub result: String,
}

/// 复制工作表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId:copyTo
pub async fn copy_to(
    request: CopyToRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CopyToResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/copyTo",
        config.base_url, request.spreadsheet_token, request.sheet_id
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
    async fn test_copy_to() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CopyToRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            destination_spreadsheet_token: "dest_spreadsheet_token".to_string(),
        };

        let result = copy_to(request, &config, None).await;
        assert!(result.is_ok());
    }
}
