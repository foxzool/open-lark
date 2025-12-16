use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 增加行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 维度信息
    pub dimension: serde_json::Value,
}

impl AddDimensionRangeRequest {
    /// 创建新的 AddDimensionRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, dimension: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            dimension,
        }
    }
}

/// 增加行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeResponse {
    /// 增加的行数
    pub addCount: i32,
    /// 维度类型
    pub majorDimension: String,
}

impl ApiResponseTrait for AddDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加行列
///
/// 根据 spreadsheetToken 和长度，在末尾增加空行/列。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/add-rows-or-columns
pub async fn add_dimension_range(
    request: AddDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<AddDimensionRangeResponse>> {
    // CSV says `post#.../dimension_range`. Endpoint name `AddDimensionRange` or `DimensionRange` with POST?
    // Checking api_endpoints.rs variants via name guess or similar pattern.
    // If I used DimensionRange for PUT, maybe there is another variant for POST?
    // In step 91 grep: `CcmSheetApiOld::DimensionRange(spreadsheet_token)`. 
    // And `to_url` for it (lines 1116) formats `/dimension_range`.
    // It seems the SAME variant is used for multiple methods (PUT, POST, DELETE).
    // So I can use `CcmSheetApiOld::DimensionRange`.
    let api_endpoint = CcmSheetApiOld::DimensionRange(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<AddDimensionRangeResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_add_dimension_range_request() {
        let request = AddDimensionRangeRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
