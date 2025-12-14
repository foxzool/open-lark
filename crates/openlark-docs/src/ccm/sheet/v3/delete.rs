use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 删除工作表请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
}

/// 删除工作表响应
#[derive(Debug, Deserialize, Default)]
pub struct DeleteSheetResponse {
    /// 是否成功
    pub success: bool,
    /// 操作结果
    pub result: String,
}

/// 删除工作表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId
pub async fn delete_sheet(
    request: DeleteSheetRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeleteSheetResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}",
        config.base_url, request.spreadsheet_token, request.sheet_id
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::DELETE,
        headers: vec![],
        query_params: vec![],
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_delete_sheet() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = DeleteSheetRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
        };

        let result = delete_sheet(request, &config, None).await;
        assert!(result.is_ok());
    }
}