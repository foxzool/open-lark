use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 添加工作表保护请求
#[derive(Debug, Serialize, Default)]
pub struct AddSheetProtectionRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 保护范围
    pub range: Option<String>,
    /// 保护类型
    pub protection_type: String,
    /// 密码
    pub password: Option<String>,
    /// 提示
    pub hint: Option<String>,
}

/// 添加工作表保护响应
#[derive(Debug, Deserialize, Default)]
pub struct AddSheetProtectionResponse {
    /// 保护ID
    pub protection_id: String,
    /// 操作结果
    pub result: String,
}

/// 添加工作表保护
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheetProtection
pub async fn add_sheet_protection(
    request: AddSheetProtectionRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<AddSheetProtectionResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheetProtection",
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
    async fn test_add_sheet_protection() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = AddSheetProtectionRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            range: Some("A1:Z100".to_string()),
            protection_type: "EDIT".to_string(),
            password: Some("protect123".to_string()),
            hint: Some("请输入保护密码".to_string()),
        };

        let result = add_sheet_protection(request, &config, None).await;
        assert!(result.is_ok());
    }
}