use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 读取单个范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuesRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    #[serde(skip)]
    pub range: String,
    /// 渲染选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    /// 日期渲染选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
}

impl GetValuesRangeRequest {
    /// 创建新的 GetValuesRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            valueRenderOption: None,
            dateTimeRenderOption: None,
        }
    }

    /// 设置 valueRenderOption
    pub fn value_render_option(mut self, option: impl Into<String>) -> Self {
        self.valueRenderOption = Some(option.into());
        self
    }
    
    /// 设置 dateTimeRenderOption
    pub fn date_time_render_option(mut self, option: impl Into<String>) -> Self {
        self.dateTimeRenderOption = Some(option.into());
        self
    }
}

/// 读取单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuesRangeResponse {
    /// 版本号
    pub revision: i64,
    /// 表格 token
    pub spreadsheetToken: String,
    /// 范围值
    pub valueRange: serde_json::Value,
}

impl ApiResponseTrait for GetValuesRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取单个范围
///
/// 根据 spreadsheetToken 和 range 读取表格单个范围的值。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
pub async fn values_range(
    request: GetValuesRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetValuesRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesRange(
        request.spreadsheet_token.clone(),
        request.range.clone(),
    );
    let mut api_request: ApiRequest<GetValuesRangeResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = &request.valueRenderOption {
        api_request = api_request.query("valueRenderOption", opt);
    }
    if let Some(opt) = &request.dateTimeRenderOption {
        api_request = api_request.query("dateTimeRenderOption", opt);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values_range_request() {
        let request = GetValuesRangeRequest::new("spreadsheet_token", "range");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
