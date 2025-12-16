use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 追加数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    pub valueRange: serde_json::Value,
}

impl AppendValuesRequest {
    /// 创建新的 AppendValuesRequest
    pub fn new(spreadsheet_token: impl Into<String>, value_range: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            valueRange: value_range,
        }
    }
}

/// 追加数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesResponse {
    /// 版本号
    pub revision: i64,
    /// 表格 token
    pub spreadsheetToken: String,
    /// 更新的范围
    pub tableRange: String,
    /// 插入的行数
    pub updates: serde_json::Value,
}

impl ApiResponseTrait for AppendValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 追加数据
///
/// 根据 spreadsheetToken 和 range 遇到空行则进行覆盖追加或新增行追加数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/append-data
pub async fn append_values(
    request: AppendValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<AppendValuesResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesAppend(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<AppendValuesResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_append_values_request() {
        let request = AppendValuesRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
