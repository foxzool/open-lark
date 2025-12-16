use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 删除保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteProtectedRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 保护范围ID列表
    pub protectIds: Vec<String>,
}

impl BatchDeleteProtectedRangeRequest {
    /// 创建新的 BatchDeleteProtectedRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, protect_ids: Vec<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            protectIds: protect_ids,
        }
    }
}

/// 删除保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteProtectedRangeResponse {
    /// 删除的保护范围ID列表
    pub delProtectIds: Vec<String>,
}

impl ApiResponseTrait for BatchDeleteProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除保护范围
///
/// 根据保护范围ID删除保护范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/delete-protection-scopes
pub async fn protected_range_batch_delete(
    request: BatchDeleteProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchDeleteProtectedRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchDel(request.spreadsheet_token.clone());
    // Note: Assuming JSON body for DELETE request as per some other APIs, but doc says DELETE and usually query params?
    // Doc path: DELETE:/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del
    // Usually batch delete takes a body with IDs.
    // Let's use delete() with body.
    let mut api_request: ApiRequest<BatchDeleteProtectedRangeResponse> = ApiRequest::delete(&api_endpoint.to_url())
        .body(serde_json::json!({
            "protectIds": request.protectIds
        }));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_range_batch_delete_request() {
        let request = BatchDeleteProtectedRangeRequest::new("spreadsheet_token", vec!["id1".to_string()]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.protectIds, vec!["id1".to_string()]);
    }
}
