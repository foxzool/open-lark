use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 修改保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateProtectedRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 请求体
    pub requests: Vec<serde_json::Value>,
}

impl BatchUpdateProtectedRangeRequest {
    /// 创建新的 BatchUpdateProtectedRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, requests: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            requests,
        }
    }
}

/// 修改保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateProtectedRangeResponse {
    /// 响应内容
    pub replies: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchUpdateProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改保护范围
///
/// 根据保护范围ID修改保护范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/modify-protection-scopes
pub async fn protected_range_batch_update(
    request: BatchUpdateProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchUpdateProtectedRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchUpdate(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchUpdateProtectedRangeResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_protected_range_batch_update_request() {
        let request = BatchUpdateProtectedRangeRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.requests.is_empty());
    }
}
