use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 维度信息
    pub dimension: serde_json::Value,
}

impl UpdateDimensionRangeRequest {
    /// 创建新的 UpdateDimensionRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, dimension: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            dimension,
        }
    }
}

/// 更新行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeResponse {
    /// 响应消息
    pub message: String,
}

impl ApiResponseTrait for UpdateDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新行列
///
/// 更新行列。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col-structure/update-row-or-column-properties
pub async fn dimension_range(
    request: UpdateDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateDimensionRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::DimensionRange(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<UpdateDimensionRangeResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    fn test_update_dimension_range_request() {
        let request = UpdateDimensionRangeRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
