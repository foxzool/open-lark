use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取电子表格信息请求
#[derive(Debug, Serialize, Default)]
pub struct GetSpreadsheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
}

/// 获取电子表格信息响应
#[derive(Debug, Deserialize, Default)]
pub struct GetSpreadsheetResponse {
    /// 电子表格信息
    pub spreadsheet: SpreadsheetInfo,
    /// 操作结果
    pub result: String,
}

/// 电子表格信息
#[derive(Debug, Deserialize, Default)]
pub struct SpreadsheetInfo {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
    /// 时区
    pub time_zone: String,
    /// 语言区域
    pub locale: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
}

/// 获取电子表格信息
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheet_token
pub async fn get_spreadsheet(
    request: GetSpreadsheetRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetSpreadsheetResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}",
        config.base_url, request.spreadsheet_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
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
    async fn test_get_spreadsheet() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetSpreadsheetRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
        };

        let result = get_spreadsheet(request, &config, None).await;
        assert!(result.is_ok());
    }
}