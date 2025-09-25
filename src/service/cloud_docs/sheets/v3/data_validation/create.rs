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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};

impl SpreadsheetSheetService {
    /// 设置下拉列表
    pub async fn set_data_validation(
        &self,
        request: SetDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SetDataValidationResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DATA_VALIDATION
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 设置下拉列表请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SetDataValidationRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 数据校验设置
    data_validation: DataValidationRule,
}

impl SetDataValidationRequest {
    pub fn builder() -> SetDataValidationRequestBuilder {
        SetDataValidationRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SetDataValidationRequestBuilder {
    request: SetDataValidationRequest,
}

impl SetDataValidationRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn data_validation(mut self, data_validation: DataValidationRule) -> Self {
        self.request.data_validation = data_validation;
        self
    }

    pub fn build(mut self) -> SetDataValidationRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 数据校验规则
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DataValidationRule {
    /// 数据校验类型
    pub condition_type: String,
    /// 应用范围
    pub range: String,
    /// 校验条件值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_values: Option<Vec<String>>,
    /// 是否拒绝输入
    #[serde(default)]
    pub strict: bool,
    /// 输入提示消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message: Option<String>,
    /// 错误提示消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 数据校验 ID（仅在响应时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_validation_id: Option<String>,
}

impl DataValidationRule {
    /// 创建下拉列表校验
    pub fn dropdown(range: impl ToString, options: Vec<String>) -> Self {
        Self {
            condition_type: "dropdown".to_string(),
            range: range.to_string(),
            condition_values: Some(options),
            strict: true,
            input_message: None,
            error_message: None,
            data_validation_id: None,
        }
    }

    /// 创建数字范围校验
    pub fn number_range(range: impl ToString, min: f64, max: f64) -> Self {
        Self {
            condition_type: "number_between".to_string(),
            range: range.to_string(),
            condition_values: Some(vec![min.to_string(), max.to_string()]),
            strict: true,
            input_message: None,
            error_message: None,
            data_validation_id: None,
        }
    }

    /// 创建文本长度校验
    pub fn text_length(range: impl ToString, min_length: u32, max_length: u32) -> Self {
        Self {
            condition_type: "text_length".to_string(),
            range: range.to_string(),
            condition_values: Some(vec![min_length.to_string(), max_length.to_string()]),
            strict: true,
            input_message: None,
            error_message: None,
            data_validation_id: None,
        }
    }

    /// 设置输入提示
    pub fn with_input_message(mut self, message: impl ToString) -> Self {
        self.input_message = Some(message.to_string());
        self
    }

    /// 设置错误提示
    pub fn with_error_message(mut self, message: impl ToString) -> Self {
        self.error_message = Some(message.to_string());
        self
    }

    /// 设置是否严格模式
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

/// 设置下拉列表响应体最外层
#[derive(Deserialize, Debug)]
pub struct SetDataValidationResponseData {
    /// 数据校验 ID
    pub data_validation_id: String,
    /// 数据校验规则信息
    #[serde(flatten)]
    pub data_validation: DataValidationRule,
}

impl ApiResponseTrait for SetDataValidationResponseData {
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
    fn test_data_validation_rule_creation() {
        let validation =
            DataValidationRule::dropdown("A1:A10", vec!["选项1".to_string(), "选项2".to_string()]);
        assert_eq!(validation.condition_type, "dropdown");
        assert_eq!(validation.range, "A1:A10");
        assert!(validation.strict);
        assert_eq!(validation.condition_values.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_set_data_validation_response() {
        let json = json!({
            "data_validation_id": "dv_001",
            "condition_type": "dropdown",
            "range": "A1:A10",
            "condition_values": ["选项1", "选项2"],
            "strict": true,
            "input_message": "请选择一个选项",
            "error_message": "输入无效"
        });

        let response: SetDataValidationResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.data_validation_id, "dv_001");
        assert_eq!(response.data_validation.condition_type, "dropdown");
    }
}

// 实现ExecutableBuilder trait
impl_executable_builder_owned!(
    SetDataValidationRequestBuilder,
    SpreadsheetSheetService,
    SetDataValidationRequest,
    BaseResponse<SetDataValidationResponseData>,
    set_data_validation
);
