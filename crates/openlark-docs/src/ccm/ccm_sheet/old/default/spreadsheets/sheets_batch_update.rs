use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 操作工作表/更新工作表属性请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetsBatchUpdateRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 请求体，包含各种操作
    pub requests: Vec<serde_json::Value>,
}

impl SheetsBatchUpdateRequest {
    /// 创建新的 SheetsBatchUpdateRequest
    pub fn new(spreadsheet_token: impl Into<String>, requests: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            requests,
        }
    }
}

/// 操作工作表/更新工作表属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetsBatchUpdateResponse {
    /// 响应列表
    pub replies: Vec<serde_json::Value>,
}

impl ApiResponseTrait for SheetsBatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 操作工作表/更新工作表属性
///
/// 根据 spreadsheetToken 更新工作表属性或执行其他操作。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets
pub async fn sheets_batch_update(
    request: SheetsBatchUpdateRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SheetsBatchUpdateResponse>> {
    // Assuming SheetsBatchUpdate variant exists. If not, I'll need to check api_endpoints.rs again or add it.
    // Based on convention: CcmSheetApiOld::SheetsBatchUpdate
    // Inspecting api_endpoints.rs earlier, I didn't see SheetsBatchUpdate explicitly listed in my grep, but I saw 'OperateSheets' earlier?
    // Let's check api_endpoints.rs content I have available or try to compile.
    // Wait, step 91 grep showed: `CcmSheetApiOld::OperateSheets`.
    // And CSV says Name is `sheets_batch_update`.
    // So I should use `CcmSheetApiOld::OperateSheets`.
    let api_endpoint = CcmSheetApiOld::OperateSheets(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<SheetsBatchUpdateResponse> = ApiRequest::post(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheets_batch_update_request() {
        let request = SheetsBatchUpdateRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.requests.is_empty());
    }
}
