use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 向多个范围写入数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateValuesRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围值列表
    pub valueRanges: Vec<serde_json::Value>,
}

impl BatchUpdateValuesRequest {
    /// 创建新的 BatchUpdateValuesRequest
    pub fn new(spreadsheet_token: impl Into<String>, value_ranges: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            valueRanges: value_ranges,
        }
    }
}

/// 向多个范围写入数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateValuesResponse {
    /// 响应列表
    pub responses: Vec<serde_json::Value>,
    /// 版本号
    pub revision: i64,
    /// 表格 token
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for BatchUpdateValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 向多个范围写入数据
///
/// 根据 spreadsheetToken 和 range 向多个范围写入数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-multiple-ranges
pub async fn values_batch_update(
    request: BatchUpdateValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchUpdateValuesResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesBatchUpdate(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchUpdateValuesResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_values_batch_update_request() {
        let request = BatchUpdateValuesRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.valueRanges.is_empty());
    }
}
