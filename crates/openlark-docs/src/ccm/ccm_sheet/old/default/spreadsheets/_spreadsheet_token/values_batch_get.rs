//! 读取多个范围
//!
//! docPath: /document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;
use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesRequest {
    /// 多个查询范围，范围之间使用逗号分隔
    pub ranges: String, // Query param: ranges=Range1,Range2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
    /// 当单元格中包含 @用户等元素时，指定返回的用户 ID 类型（open_id / union_id 等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    #[serde(default)]
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    #[serde(default)]
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchGetValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取多个范围
pub async fn values_batch_get(
    spreadsheet_token: String,
    request: BatchGetValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchGetValuesResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.ranges, "ranges 不能为空");

    let api_endpoint = CcmSheetApiOld::ValuesBatchGet(spreadsheet_token);
    let mut api_request: ApiRequest<BatchGetValuesResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query("ranges", request.ranges)
            .query_opt("valueRenderOption", request.valueRenderOption)
            .query_opt("dateTimeRenderOption", request.dateTimeRenderOption)
            .query_opt("user_id_type", request.user_id_type);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "读取多个范围")
}
