use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetProtectedRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 保护范围ID列表
    pub protectIds: Vec<String>,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberType: Option<String>,
}

impl BatchGetProtectedRangeRequest {
    /// 创建新的 BatchGetProtectedRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, protect_ids: Vec<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            protectIds: protect_ids,
            memberType: None,
        }
    }

    /// 设置 memberType
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.memberType = Some(member_type.into());
        self
    }
}

/// 获取保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetProtectedRangeResponse {
    /// 保护范围列表
    pub protectedRange: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchGetProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取保护范围
///
/// 根据保护范围ID查询详细的保护行列信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/retrieve-protection-scopes
pub async fn protected_range_batch_get(
    request: BatchGetProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchGetProtectedRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchGet(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchGetProtectedRangeResponse> = ApiRequest::get(&api_endpoint.to_url())
        .query("protectIds", &request.protectIds.join(",")); // Assuming query param format

    if let Some(member_type) = &request.memberType {
        api_request = api_request.query("memberType", member_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_range_batch_get_request() {
        let request = BatchGetProtectedRangeRequest::new("spreadsheet_token", vec!["id1".to_string()]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.protectIds, vec!["id1".to_string()]);
    }
}
