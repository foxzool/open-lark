/// 设置数据验证
///
/// 为电子表格中的单元格区域设置数据验证规则，限制用户输入。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dataValidation
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiV3, api_utils::*};

/// 设置数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataValidationRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 验证范围
    pub range: String,
    /// 验证条件
    pub condition: ValidationCondition,
    /// 输入消息
    pub input_message: Option<String>,
    /// 错误消息
    pub error_message: Option<String>,
    /// 是否显示错误提示
    pub strict: Option<bool>,
}

impl SetDataValidationRequest {
    /// 创建设置数据验证请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    /// * `range` - 验证范围
    /// * `condition` - 验证条件
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: impl Into<String>,
        condition: ValidationCondition,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            range: range.into(),
            condition,
            input_message: None,
            error_message: None,
            strict: None,
        }
    }

    /// 设置输入消息
    pub fn input_message(mut self, message: impl Into<String>) -> Self {
        self.input_message = Some(message.into());
        self
    }

    /// 设置错误消息
    pub fn error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    /// 设置是否严格模式
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }
}

/// 验证条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    /// 验证类型
    pub r#type: String,
    /// 值
    pub values: Vec<serde_json::Value>,
    /// 比较类型
    pub compare_type: Option<String>,
}

impl ValidationCondition {
    /// 创建验证条件
    ///
    /// # 参数
    /// * `validation_type` - 验证类型
    /// * `values` - 验证值列表
    pub fn new(
        validation_type: impl Into<String>,
        values: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            r#type: validation_type.into(),
            values,
            compare_type: None,
        }
    }

    /// 设置比较类型
    pub fn compare_type(mut self, compare_type: impl Into<String>) -> Self {
        self.compare_type = Some(compare_type.into());
        self
    }
}

/// 设置数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataValidationResponse {
    /// 验证信息
    pub data: Option<DataValidationInfo>,
}

impl ApiResponseTrait for SetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据验证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationInfo {
    /// 验证ID
    pub validation_id: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 验证范围
    pub range: String,
    /// 验证条件
    pub condition: ValidationConditionInfo,
}

/// 验证条件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConditionInfo {
    /// 验证类型
    pub r#type: String,
    /// 值
    pub values: Vec<serde_json::Value>,
    /// 比较类型
    pub compare_type: Option<String>,
}

/// 设置数据验证
///
/// 为电子表格中的单元格区域设置数据验证规则，限制用户输入。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dataValidation
pub async fn set_data_validation(
    request: SetDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SetDataValidationResponse>> {
    // 构建请求体
    let mut body = json!({
        "sheetId": request.sheet_id,
        "range": request.range,
        "condition": request.condition
    });

    if let Some(input_message) = &request.input_message {
        body["inputMessage"] = json!(input_message);
    }
    if let Some(error_message) = &request.error_message {
        body["errorMessage"] = json!(error_message);
    }
    if let Some(strict) = request.strict {
        body["strict"] = json!(strict);
    }

    // 创建API请求
    let mut api_request: ApiRequest<SetDataValidationResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/dataValidation", CcmSheetApiV3, request.spreadsheet_token))
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
    fn test_set_data_validation_request_builder() {
        let condition = ValidationCondition::new(
            "NUMBER_BETWEEN",
            vec![
                serde_json::json!(1),
                serde_json::json!(100),
            ]
        );

        let request = SetDataValidationRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "A1:A10",
            condition
        );

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
        assert_eq!(request.range, "A1:A10");
        assert_eq!(request.condition.r#type, "NUMBER_BETWEEN");
    }

    #[test]
    fn test_set_data_validation_request_builder_chain() {
        let condition = ValidationCondition::new(
            "NUMBER_BETWEEN",
            vec![
                serde_json::json!(1),
                serde_json::json!(100),
            ]
        );

        let request = SetDataValidationRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "A1:A10",
            condition
        )
        .input_message("请输入1-100之间的数字")
        .error_message("输入的数字不在有效范围内")
        .strict(true);

        assert_eq!(request.input_message, Some("请输入1-100之间的数字".to_string()));
        assert_eq!(request.error_message, Some("输入的数字不在有效范围内".to_string()));
        assert_eq!(request.strict, Some(true));
    }

    #[test]
    fn test_validation_condition_builder() {
        let condition = ValidationCondition::new(
            "TEXT_CONTAINS",
            vec![serde_json::json!("测试")]
        )
        .compare_type("EQUALS");

        assert_eq!(condition.r#type, "TEXT_CONTAINS");
        assert_eq!(condition.values[0], serde_json::json!("测试"));
        assert_eq!(condition.compare_type, Some("EQUALS".to_string()));
    }

    #[test]
    fn test_data_validation_info_structure() {
        let validation_info = DataValidationInfo {
            validation_id: "validation_123".to_string(),
            sheet_id: "sheet_456".to_string(),
            range: "A1:C10".to_string(),
            condition: ValidationConditionInfo {
                r#type: "NUMBER_BETWEEN".to_string(),
                values: vec![serde_json::json!(1), serde_json::json!(100)],
                compare_type: None,
            },
        };

        assert_eq!(validation_info.validation_id, "validation_123");
        assert_eq!(validation_info.sheet_id, "sheet_456");
        assert_eq!(validation_info.range, "A1:C10");
        assert_eq!(validation_info.condition.r#type, "NUMBER_BETWEEN");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SetDataValidationResponse::data_format(), ResponseFormat::Data);
    }
}