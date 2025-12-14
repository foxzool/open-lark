use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 设置数据验证请求
#[derive(Debug, Serialize, Default)]
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

/// 验证条件
#[derive(Debug, Serialize, Default)]
pub struct ValidationCondition {
    /// 验证类型
    pub r#type: String,
    /// 值
    pub values: Vec<serde_json::Value>,
    /// 比较类型
    pub compare_type: Option<String>,
}

/// 设置数据验证响应
#[derive(Debug, Deserialize, Default)]
pub struct SetDataValidationResponse {
    /// 验证信息
    pub data_validation: DataValidationInfo,
    /// 操作结果
    pub result: String,
}

/// 数据验证信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct ValidationConditionInfo {
    /// 验证类型
    pub r#type: String,
    /// 值
    pub values: Vec<serde_json::Value>,
    /// 比较类型
    pub compare_type: Option<String>,
}

/// 设置数据验证
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/dataValidation
pub async fn set_data_validation(
    request: SetDataValidationRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SetDataValidationResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/dataValidation",
        config.base_url, request.spreadsheet_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_set_data_validation() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = SetDataValidationRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            range: "A1:A10".to_string(),
            condition: ValidationCondition {
                r#type: "NUMBER_BETWEEN".to_string(),
                values: vec![
                    serde_json::Value::Number(serde_json::Number::from(1)),
                    serde_json::Value::Number(serde_json::Number::from(100)),
                ],
                compare_type: None,
            },
            input_message: Some("请输入1-100之间的数字".to_string()),
            error_message: Some("输入的数字不在有效范围内".to_string()),
            strict: Some(true),
        };

        let result = set_data_validation(request, &config, None).await;
        assert!(result.is_ok());
    }
}