use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 修改电子表格属性请求
#[derive(Debug, Serialize, Default)]
pub struct PatchSpreadsheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言区域
    pub locale: Option<String>,
    /// 更新字段
    pub fields: Vec<String>,
}

/// 修改电子表格属性响应
#[derive(Debug, Deserialize, Default)]
pub struct PatchSpreadsheetResponse {
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

/// 修改电子表格属性
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheet_token
pub async fn patch_spreadsheet(
    request: PatchSpreadsheetRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<PatchSpreadsheetResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}",
        config.base_url, request.spreadsheet_token
    );

    let mut query_params = vec![];

    if !request.fields.is_empty() {
        query_params.push(("fields".to_string(), request.fields.join(",")));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::PATCH,
        headers: vec![],
        query_params,
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_patch_spreadsheet() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = PatchSpreadsheetRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            title: Some("修改后的标题".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
            locale: None,
            fields: vec!["title".to_string(), "time_zone".to_string()],
        };

        let result = patch_spreadsheet(request, &config, None).await;
        assert!(result.is_ok());
    }
}