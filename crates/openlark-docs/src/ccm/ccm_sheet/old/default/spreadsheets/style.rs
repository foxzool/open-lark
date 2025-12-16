use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 设置单元格样式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 附加样式
    pub appendStyle: Option<serde_json::Value>,
    /// 范围
    pub range: String,
    /// 样式
    pub style: serde_json::Value,
}

impl SetStyleRequest {
    /// 创建新的 SetStyleRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>, style: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            appendStyle: None,
            range: range.into(),
            style,
        }
    }

    /// 设置附加样式
    pub fn append_style(mut self, append_style: serde_json::Value) -> Self {
        self.appendStyle = Some(append_style);
        self
    }
}

/// 设置单元格样式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleResponse {
    /// 表格 token
    pub spreadsheetToken: String,
    /// 更新的单元格数
    pub updatedCells: i32,
    /// 更新的列数
    pub updatedColumns: i32,
    /// 更新的范围
    pub updatedRange: String,
    /// 更新的行数
    pub updatedRows: i32,
    /// 版本号
    pub revision: i64,
}

impl ApiResponseTrait for SetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置单元格样式
///
/// 根据 spreadsheetToken 、range 和样式信息更新单元格样式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style
pub async fn set_style(
    request: SetStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SetStyleResponse>> {
    let api_endpoint = CcmSheetApiOld::Style(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<SetStyleResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    fn test_set_style_request() {
        let request = SetStyleRequest::new("spreadsheet_token", "range", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
