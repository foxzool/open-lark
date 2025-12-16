use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 删除Sheet请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
}

impl DeleteSheetRequest {
    /// 创建新的 DeleteSheetRequest
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
        }
    }
}

/// 删除Sheet响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSheetResponse {
    /// id
    pub id: String,
    /// result
    pub result: bool,
}

impl ApiResponseTrait for DeleteSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除Sheet
///
/// 根据 spreadsheetToken 删除对应的 sheet 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-sheet
pub async fn delete(
    request: DeleteSheetRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteSheetResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::FileSpreadsheets(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<DeleteSheetResponse> = ApiRequest::delete(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_sheet_request() {
        let request = DeleteSheetRequest::new("spreadsheet_token");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
