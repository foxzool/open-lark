use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::SheetsApiV3;

/// 删除工作表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
}

impl DeleteSheetRequest {
    /// 创建删除工作表请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = sheet_id.into();
        self
    }
}

/// 删除工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetResponse {
    /// 是否成功
    pub success: bool,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for DeleteSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除工作表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId
pub async fn delete_sheet(
    request: DeleteSheetRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteSheetResponse>> {
    // 构建API端点URL
    let url = format!(
        "{}/spreadsheets/{}/sheets/{}",
        SheetsApiV3, request.spreadsheet_token, request.sheet_id
    );

    // 创建API请求
    let mut api_request: ApiRequest<DeleteSheetResponse> = ApiRequest::delete(&url);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_sheet_request_builder() {
        let request = DeleteSheetRequest::new("spreadsheet_token", "sheet_id");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
    }

    #[test]
    fn test_delete_sheet_request_with_token() {
        let request = DeleteSheetRequest::new("initial_token", "initial_id")
            .spreadsheet_token("new_token")
            .sheet_id("new_id");

        assert_eq!(request.spreadsheet_token, "new_token");
        assert_eq!(request.sheet_id, "new_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteSheetResponse::data_format(), ResponseFormat::Data);
    }
}