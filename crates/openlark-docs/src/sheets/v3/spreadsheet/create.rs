use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建电子表格请求
#[derive(Debug, Serialize, Default)]
pub struct CreateSpreadsheetRequest {
    /// 表格标题
    pub title: String,
    /// 文件夹token
    pub folder_token: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言区域
    pub locale: Option<String>,
}

/// 创建电子表格响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateSpreadsheetResponse {
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
}

/// 创建电子表格
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/create
pub async fn create_spreadsheet(
    request: CreateSpreadsheetRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateSpreadsheetResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets",
        config.base_url
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
    async fn test_create_spreadsheet() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateSpreadsheetRequest {
            title: "新建电子表格".to_string(),
            folder_token: Some("test_folder_token".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
            locale: Some("zh_CN".to_string()),
        };

        let result = create_spreadsheet(request, &config, None).await;
        assert!(result.is_ok());
    }
}