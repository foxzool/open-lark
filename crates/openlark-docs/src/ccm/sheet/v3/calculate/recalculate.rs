/// 重新计算
///
/// 重新计算电子表格中的公式。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/recalculate
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 重新计算请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecalculateRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID（可选，不指定则计算全部）
    pub sheet_id: Option<String>,
    /// 计算范围（可选）
    pub range: Option<String>,
}

impl RecalculateRequest {
    /// 创建重新计算请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: None,
            range: None,
        }
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = Some(sheet_id.into());
        self
    }

    /// 设置计算范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }
}

/// 重新计算响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecalculateResponse {
    /// 计算ID
    pub calculation_id: String,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for RecalculateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 重新计算
///
/// 重新计算电子表格中的公式。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/recalculate
pub async fn recalculate(
    request: RecalculateRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<RecalculateResponse>> {
    // 构建请求体
    let mut body = json!({});

    if let Some(sheet_id) = &request.sheet_id {
        body["sheetId"] = json!(sheet_id);
    }
    if let Some(range) = &request.range {
        body["range"] = json!(range);
    }

    // 创建API请求
    let mut api_request: ApiRequest<RecalculateResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/recalculate", SheetsApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recalculate_request_builder() {
        let request = RecalculateRequest::new("spreadsheet_token")
            .sheet_id("sheet_id")
            .range("A1:D10");

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, Some("sheet_id".to_string()));
        assert_eq!(request.range, Some("A1:D10".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(RecalculateResponse::data_format(), ResponseFormat::Data);
    }
}