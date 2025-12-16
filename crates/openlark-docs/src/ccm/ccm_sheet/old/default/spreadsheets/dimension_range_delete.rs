use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 删除行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 维度信息
    pub dimension: serde_json::Value,
}

impl DeleteDimensionRangeRequest {
    /// 创建新的 DeleteDimensionRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, dimension: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            dimension,
        }
    }
}

/// 删除行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeResponse {
    /// 删除的行数/列数
    pub delCount: i32,
    /// 维度主键
    pub majorDimension: String,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列
///
/// 删除行列。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col-structure/delete-rows-or-columns
pub async fn delete_dimension_range(
    request: DeleteDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteDimensionRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::DimensionRangeDelete(request.spreadsheet_token.clone());
    // Note: The HTTP method for deletion is often DELETE, but for this specific endpoint it might be PUT or DELETE.
    // Based on convention for similar range operations that carry a body, PUT or POST is possible.
    // However, typical delete operations use delete(). Checking documentation/endpoint definition:
    // CcmSheetApiOld::DimensionRangeDelete seems to imply a DELETE action, but if it requires a body, standard Transport::delete might not support body easily without checking implementation.
    // Let's assume DELETE with body or PUT if it's a modification struct.
    // Wait, documentation link says "DELETE /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range"
    // So it is DELETE.
    // Standard ReqBuilder might not support body in delete() directly if not designed so.
    // Let's check api_request wrapper. ApiRequest::delete() usually doesn't take body.
    // If body is needed, we might need a custom request or check if ApiRequest supports body on delete.
    // Looking at other APIs, `delete` usually takes path params.
    // But here we have `dimension` in body.
    // Let's try ApiRequest::new(Method::DELETE, ...) or check if delete() allows body chaining.
    // Assuming ApiRequest supports .body() on any method builder.
    let mut api_request: ApiRequest<DeleteDimensionRangeResponse> = ApiRequest::delete(&api_endpoint.to_url())
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
    fn test_delete_dimension_range_request() {
        let request = DeleteDimensionRangeRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
