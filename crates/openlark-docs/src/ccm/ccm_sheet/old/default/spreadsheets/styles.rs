use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量设置单元格样式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStylesRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 样式数据
    pub data: Vec<serde_json::Value>,
}

impl BatchUpdateStylesRequest {
    /// 创建新的 BatchUpdateStylesRequest
    pub fn new(spreadsheet_token: impl Into<String>, data: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            data,
        }
    }
}

/// 批量设置单元格样式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStylesResponse {
    /// 表格 token
    pub spreadsheetToken: String,
    /// 更新的总单元格数
    pub totalUpdatedCells: i32,
    /// 版本号
    pub revision: i64,
    /// 响应数据
    pub responses: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchUpdateStylesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量设置单元格样式
///
/// 批量设置单元格样式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style
pub async fn batch_update_styles(
    request: BatchUpdateStylesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchUpdateStylesResponse>> {
    let api_endpoint = CcmSheetApiOld::StylesBatchUpdate(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<BatchUpdateStylesResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    fn test_batch_update_styles_request() {
        let request = BatchUpdateStylesRequest::new("spreadsheet_token", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.data.is_empty());
    }
}
