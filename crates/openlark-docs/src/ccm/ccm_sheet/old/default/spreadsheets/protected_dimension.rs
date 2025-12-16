use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 增加保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedDimensionRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 保护范围信息
    pub addProtectedDimension: Vec<serde_json::Value>,
}

impl ProtectedDimensionRequest {
    /// 创建新的 ProtectedDimensionRequest
    pub fn new(spreadsheet_token: impl Into<String>, add_protected_dimension: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            addProtectedDimension: add_protected_dimension,
        }
    }
}

/// 增加保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedDimensionResponse {
    /// 响应体
    pub addProtectedDimension: Vec<serde_json::Value>,
}

impl ApiResponseTrait for ProtectedDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加保护范围
///
/// 增加保护范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/add-lock
pub async fn protected_dimension(
    request: ProtectedDimensionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ProtectedDimensionResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedDimension(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<ProtectedDimensionResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_protected_dimension_request() {
        let request = ProtectedDimensionRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.addProtectedDimension.is_empty());
    }
}
