use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 合并单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    pub range: String,
    /// 合并类型，可选 "MERGE_ALL", "MERGE_ROWS", "MERGE_COLUMNS"
    pub mergeType: String,
}

impl MergeCellsRequest {
    /// 创建新的 MergeCellsRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>, merge_type: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            mergeType: merge_type.into(),
        }
    }
}

/// 合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResponse {
    /// 表格 token
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 合并单元格
///
/// 合并单元格。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells
pub async fn merge_cells(
    request: MergeCellsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<MergeCellsResponse>> {
    let api_endpoint = CcmSheetApiOld::MergeCells(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<MergeCellsResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_merge_cells_request() {
        let request = MergeCellsRequest::new("spreadsheet_token", "range", "MERGE_ALL");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
        assert_eq!(request.mergeType, "MERGE_ALL");
    }
}
