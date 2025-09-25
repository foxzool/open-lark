use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetService,
};

impl SpreadsheetSheetService {
    /// 删除下拉列表设置
    pub async fn delete_data_validation(
        &self,
        request: DeleteDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteDataValidationResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.data_validation_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除下拉列表设置请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验 ID
    data_validation_id: String,
}

impl DeleteDataValidationRequest {
    pub fn builder() -> DeleteDataValidationRequestBuilder {
        DeleteDataValidationRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteDataValidationRequestBuilder {
    request: DeleteDataValidationRequest,
}

impl DeleteDataValidationRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn data_validation_id(mut self, data_validation_id: impl ToString) -> Self {
        self.request.data_validation_id = data_validation_id.to_string();
        self
    }

    pub fn build(mut self) -> DeleteDataValidationRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 删除下拉列表设置响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteDataValidationResponseData {
    /// 删除操作是否成功
    #[serde(default)]
    pub success: bool,
    /// 删除的数据校验 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_validation_id: Option<String>,
}

impl ApiResponseTrait for DeleteDataValidationResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_delete_data_validation_response() {
        let json = json!({
            "success": true,
            "data_validation_id": "dv_001"
        });

        let response: DeleteDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.data_validation_id, Some("dv_001".to_string()));
    }
}
