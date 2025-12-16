use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataValidationRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    pub range: String,
}

impl GetDataValidationRequest {
    /// 创建新的 GetDataValidationRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
        }
    }
}

/// 获取数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataValidationResponse {
    /// 数据验证列表
    pub dataValidations: Vec<serde_json::Value>,
    /// 表格 token
    pub spreadsheetToken: String,
    /// 版本号
    pub revision: i64,
}

impl ApiResponseTrait for GetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取数据验证
///
/// 获取范围内的所有数据验证规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-datavalidation
pub async fn data_validation(
    request: GetDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetDataValidationResponse>> {
    let api_endpoint = CcmSheetApiOld::DataValidation(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<GetDataValidationResponse> = ApiRequest::get(&api_endpoint.to_url())
        .query("range", &request.range);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_validation_request() {
        let request = GetDataValidationRequest::new("spreadsheet_token", "range");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
