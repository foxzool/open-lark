use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新表格属性请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateSpreadsheetPropertiesRequest {
    /// 创建新的 UpdateSpreadsheetPropertiesRequest
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            title: None,
        }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// 更新表格属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesResponse {
    /// 表格 token
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: String,
}

impl ApiResponseTrait for UpdateSpreadsheetPropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表格属性
///
/// 根据 spreadsheetToken 更新工作表属性。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/update-sheet-properties
pub async fn update_spreadsheet_properties(
    request: UpdateSpreadsheetPropertiesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateSpreadsheetPropertiesResponse>> {
    let api_endpoint = CcmSheetApiOld::Properties(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<UpdateSpreadsheetPropertiesResponse> = ApiRequest::put(&api_endpoint.to_url())
        .body(serde_json::json!({
            "properties": {
                "title": request.title
            }
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
    fn test_update_spreadsheet_properties_request() {
        let request = UpdateSpreadsheetPropertiesRequest::new("spreadsheet_token")
            .title("new title");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.title, Some("new title".to_string()));
    }
}
