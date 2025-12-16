use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量删除条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteConditionFormatsRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// sheetConditionFormatIds
    pub sheetConditionFormatIds: Vec<String>,
}

impl BatchDeleteConditionFormatsRequest {
    /// 创建新的 BatchDeleteConditionFormatsRequest
    pub fn new(spreadsheet_token: impl Into<String>, sheet_condition_format_ids: Vec<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheetConditionFormatIds: sheet_condition_format_ids,
        }
    }
}

/// 批量删除条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteConditionFormatsResponse {
    /// 响应列表
    pub responses: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteConditionFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除条件格式
///
/// 删除已有的条件格式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-delete
pub async fn condition_formats_batch_delete(
    request: BatchDeleteConditionFormatsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchDeleteConditionFormatsResponse>> {
    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchDelete(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchDeleteConditionFormatsResponse> = ApiRequest::delete(&api_endpoint.to_url())
        .body(serde_json::json!({
            "sheetConditionFormatIds": request.sheetConditionFormatIds
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
    fn test_condition_formats_batch_delete_request() {
        let request = BatchDeleteConditionFormatsRequest::new("spreadsheet_token", vec!["id1".to_string()]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheetConditionFormatIds, vec!["id1".to_string()]);
    }
}
