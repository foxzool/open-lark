use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量创建条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateConditionFormatsRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 条件格式列表
    pub sheetConditionFormats: Vec<serde_json::Value>,
}

impl BatchCreateConditionFormatsRequest {
    /// 创建新的 BatchCreateConditionFormatsRequest
    pub fn new(spreadsheet_token: impl Into<String>, sheet_condition_formats: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheetConditionFormats: sheet_condition_formats,
        }
    }
}

/// 批量创建条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateConditionFormatsResponse {
    /// 响应列表
    pub responses: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateConditionFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建条件格式
///
/// 创建新的条件格式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-set
pub async fn condition_formats_batch_create(
    request: BatchCreateConditionFormatsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchCreateConditionFormatsResponse>> {
    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchCreate(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchCreateConditionFormatsResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_condition_formats_batch_create_request() {
        let request = BatchCreateConditionFormatsRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.sheetConditionFormats.is_empty());
    }
}
