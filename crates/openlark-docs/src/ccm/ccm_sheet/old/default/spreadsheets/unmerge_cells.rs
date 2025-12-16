use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 拆分单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    pub range: String,
}

impl UnmergeCellsRequest {
    /// 创建新的 UnmergeCellsRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
        }
    }
}

/// 拆分单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsResponse {
    /// 表格 token
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for UnmergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 拆分单元格
///
/// 拆分单元格。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/unmerge-cells
pub async fn unmerge_cells(
    request: UnmergeCellsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UnmergeCellsResponse>> {
    let api_endpoint = CcmSheetApiOld::UnmergeCells(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<UnmergeCellsResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_unmerge_cells_request() {
        let request = UnmergeCellsRequest::new("spreadsheet_token", "range");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
