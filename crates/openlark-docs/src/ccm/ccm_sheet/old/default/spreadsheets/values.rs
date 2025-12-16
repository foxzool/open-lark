use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 值范围
    pub valueRange: serde_json::Value,
}

impl ValuesRequest {
    /// 创建新的 ValuesRequest
    pub fn new(spreadsheet_token: impl Into<String>, value_range: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            valueRange: value_range,
        }
    }
}

/// 更新范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesResponse {
    /// 版本号
    pub revision: i64,
    /// 表格 token
    pub spreadsheetToken: String,
    /// 更新的单元格数量
    pub updatedCells: i32,
    /// 更新的列数
    pub updatedColumns: i32,
    /// 更新的范围
    pub updatedRange: String,
    /// 更新的行数
    pub updatedRows: i32,
}

impl ApiResponseTrait for ValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新范围
///
/// 向单个范围写入数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
pub async fn values(
    request: ValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ValuesResponse>> {
    let api_endpoint = CcmSheetApiOld::Values(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<ValuesResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    fn test_values_request() {
        let request = ValuesRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
