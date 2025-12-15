use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 复制工作表请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyToRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 源工作表ID
    pub sheet_id: String,
    /// 目标电子表格token
    pub destination_spreadsheet_token: String,
}

/// 复制工作表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CopyToResponse {
    /// 新工作表ID
    pub sheet_id: String,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for CopyToResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CopyToRequest {
    /// 创建新的复制工作表请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置源工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置目标电子表格token
    pub fn destination_spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.destination_spreadsheet_token = token.into();
        self
    }
}

/// 复制工作表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/:sheetId:copyTo
pub async fn copy_to(
    request: CopyToRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CopyToResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<CopyToResponse> =
        ApiRequest::post(&format!(
            "{}/spreadsheets/{}/sheets/{}/copyTo",
            SheetsApiV3, request.spreadsheet_token, request.sheet_id
        ))
        .body(body);

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
    fn test_copy_to_request_builder() {
        let request = CopyToRequest::new()
            .spreadsheet_token("src_token")
            .sheet_id("src_sheet")
            .destination_spreadsheet_token("dest_token");

        assert_eq!(request.spreadsheet_token, "src_token");
        assert_eq!(request.sheet_id, "src_sheet");
        assert_eq!(request.destination_spreadsheet_token, "dest_token");
    }
}