/// 更新数据验证规则
///
/// 根据 spreadsheetToken 和 dataValidationRuleId 更新数据验证规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/update-data-validation
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新数据验证规则请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDropdownParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据验证规则ID
    #[serde(rename = "dataValidationRuleId")]
    pub data_validation_rule_id: i64,
    /// 数据验证规则
    #[serde(rename = "dataValidationRule")]
    pub data_validation_rule: DataValidationRule,
}

/// 数据验证规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationRule {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 条件类型
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    /// 数据比较条件
    #[serde(rename = "condition")]
    pub condition: ValidationCondition,
    /// 是否严格模式
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// 输入提示信息
    #[serde(rename = "inputMessage", skip_serializing_if = "Option::is_none")]
    pub input_message: Option<String>,
    /// 输入提示标题
    #[serde(rename = "inputTitle", skip_serializing_if = "Option::is_none")]
    pub input_title: Option<String>,
    /// 错误信息
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误提示标题
    #[serde(rename = "errorTitle", skip_serializing_if = "Option::is_none")]
    pub error_title: Option<String>,
}

/// 数据比较条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    /// 比较类型
    #[serde(rename = "type")]
    pub condition_type: String,
    /// 条件值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// 更新数据验证规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDropdownResponse {
    /// 更新结果
    pub data: Option<UpdateDropdownResult>,
}

/// 更新数据验证规则结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDropdownResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的数据验证规则
    #[serde(rename = "dataValidationRule")]
    pub data_validation_rule: DataValidationRuleResult,
}

/// 数据验证规则结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationRuleResult {
    /// 验证规则ID
    #[serde(rename = "dataValidationRuleId")]
    pub data_validation_rule_id: i64,
    /// 验证规则信息
    #[serde(rename = "dataValidationRule")]
    pub data_validation_rule: DataValidationRule,
}

impl ApiResponseTrait for UpdateDropdownResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新数据验证规则请求
pub struct UpdateDropdownRequest {
    config: Config,
}

impl UpdateDropdownRequest {
    /// 创建更新数据验证规则请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/update-data-validation
    pub async fn execute(self, params: UpdateDropdownParams) -> SDKResult<UpdateDropdownResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::DataValidationUpdate(
            params.spreadsheet_token.clone(),
            params.data_validation_rule_id.to_string(),
        );

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateDropdownResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
