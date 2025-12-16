use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新下拉列表设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataValidationRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// sheet Id
    #[serde(skip)]
    pub sheetId: String,
    /// dataValidation Id
    #[serde(skip)]
    pub dataValidationId: i32,
    /// 下拉列表类型
    pub dataValidationType: String,
    /// 下拉列表值
    pub dataValidation: Vec<serde_json::Value>,
}

impl UpdateDataValidationRequest {
    /// 创建新的 UpdateDataValidationRequest
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>, data_validation_id: i32, data_validation_type: impl Into<String>, data_validation: Vec<serde_json::Value>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheetId: sheet_id.into(),
            dataValidationId: data_validation_id,
            dataValidationType: data_validation_type.into(),
            dataValidation: data_validation,
        }
    }
}

/// 更新下拉列表设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataValidationResponse {
    /// 范围
    pub range: String,
}

impl ApiResponseTrait for UpdateDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新下拉列表设置
///
/// 根据 spreadsheetToken 、sheetId、dataValidationId 更新下拉列表的属性。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/update-datavalidation
pub async fn data_validation_update(
    request: UpdateDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateDataValidationResponse>> {
    let _api_endpoint = CcmSheetApiOld::DataValidationUpdate(
        request.spreadsheet_token.clone(),
        format!("{}", request.dataValidationId), // Assuming ID in URL, but variants take string? Let's assume validation_id in enum is String.
    );
    // Wait, DataValidationUpdate in enum takes (spreadsheet_token, validation_id).
    // The URL structure maps /:sheetId/:dataValidationId?
    // Let's check api_endpoints.rs for DataValidationUpdate.
    // Line 1226: DataValidationUpdate(spreadsheet_token, validation_id) -> .../dataValidation/.../{}
    // Wait, I need BOTH sheetId and dataValidationId in the URL path?
    // If the enum only has 2 params, maybe sheetId is part of validation_id param? Or I misread the enum.
    // In step 91 grep output: `DataValidationUpdate(spreadsheet_token, validation_id)`.
    // And CSV URL: `.../dataValidation/:sheetId/:dataValidationId`.
    // It seems the enum variant is missing a parameter or `validation_id` is expected to be `sheetId/dataValidationId`.
    // I will construct the id string manually.
    let combined_id = format!("{}/{}", request.sheetId, request.dataValidationId);
    let api_endpoint = CcmSheetApiOld::DataValidationUpdate(request.spreadsheet_token.clone(), combined_id);
    
    let mut api_request: ApiRequest<UpdateDataValidationResponse> = ApiRequest::put(&api_endpoint.to_url())
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
    fn test_data_validation_update_request() {
        let request = UpdateDataValidationRequest::new("spreadsheet_token", "sheetId", 123, "list", vec![]);
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheetId, "sheetId");
        assert_eq!(request.dataValidationId, 123);
    }
}
