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

use super::create::DataValidationRule;

impl SpreadsheetSheetService {
    /// 更新下拉列表设置
    pub async fn update_data_validation(
        &self,
        request: UpdateDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateDataValidationResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.data_validation_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 更新下拉列表设置请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验 ID
    data_validation_id: String,
    /// 新的数据校验设置
    data_validation: DataValidationRule,
}

impl UpdateDataValidationRequest {
    pub fn builder() -> UpdateDataValidationRequestBuilder {
        UpdateDataValidationRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateDataValidationRequestBuilder {
    request: UpdateDataValidationRequest,
}

impl UpdateDataValidationRequestBuilder {
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

    pub fn data_validation(mut self, data_validation: DataValidationRule) -> Self {
        self.request.data_validation = data_validation;
        self
    }

    pub fn build(mut self) -> UpdateDataValidationRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新下拉列表设置响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateDataValidationResponseData {
    /// 数据校验 ID
    pub data_validation_id: String,
    /// 更新后的数据校验规则信息
    #[serde(flatten)]
    pub data_validation: DataValidationRule,
}

impl ApiResponseTrait for UpdateDataValidationResponseData {
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
    fn test_update_data_validation_response() {
        let json = json!({
            "data_validation_id": "dv_001",
            "condition_type": "number_between",
            "range": "B1:B10",
            "condition_values": ["1", "100"],
            "strict": true,
            "input_message": "请输入1-100之间的数字",
            "error_message": "数字超出范围"
        });

        let response: UpdateDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.data_validation_id, "dv_001");
        assert_eq!(response.data_validation.condition_type, "number_between");
    }
}
