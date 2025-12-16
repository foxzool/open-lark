use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 设置下拉列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataValidationRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 范围
    pub range: String,
    /// 下拉列表类型
    pub dataValidationType: String,
    /// 下拉列表值
    pub dataValidation: Vec<serde_json::Value>,
}

impl CreateDataValidationRequest {
    /// 创建新的 CreateDataValidationRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>, data_validation_type: impl Into<String>, data_validation: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            dataValidationType: data_validation_type.into(),
            dataValidation: data_validation,
        }
    }
}

/// 设置下拉列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataValidationResponse {
    /// 成功信息
    pub msg: String,
    /// 代码
    pub code: i32,
}

impl ApiResponseTrait for CreateDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置下拉列表
///
/// 根据 spreadsheetToken 、range 和下拉列表属性给单元格设置下拉列表规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/set-dropdown
pub async fn data_validation_create(
    request: CreateDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateDataValidationResponse>> {
    let api_endpoint = CcmSheetApiOld::DataValidationCreate(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<CreateDataValidationResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_data_validation_create_request() {
        let request = CreateDataValidationRequest::new("spreadsheet_token", "range", "list", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
