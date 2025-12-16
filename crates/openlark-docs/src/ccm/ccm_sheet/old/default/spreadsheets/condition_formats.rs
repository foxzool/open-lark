use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionFormatsRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 工作表 ID，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheetKey: Option<String>,
}

impl GetConditionFormatsRequest {
    /// 创建新的 GetConditionFormatsRequest
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheetKey: None,
        }
    }

    /// 设置工作表 ID
    pub fn sheet_key(mut self, sheet_key: impl Into<String>) -> Self {
        self.sheetKey = Some(sheet_key.into());
        self
    }
}

/// 获取条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionFormatsResponse {
    /// 条件格式列表
    pub sheetConditionFormats: Vec<serde_json::Value>,
}

impl ApiResponseTrait for GetConditionFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取条件格式
///
/// 获取表格的条件格式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/condition-format-get
pub async fn get_condition_formats(
    request: GetConditionFormatsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetConditionFormatsResponse>> {
    let api_endpoint = CcmSheetApiOld::ConditionFormats(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<GetConditionFormatsResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(sheet_key) = &request.sheetKey {
        api_request = api_request.query("sheet_ids", sheet_key); // Note: API might expect comma separated list, handled simply here
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
    fn test_get_condition_formats_request() {
        let request = GetConditionFormatsRequest::new("spreadsheet_token");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.sheetKey.is_none());
    }
}
