/// 批量创建条件格式
///
/// 根据 spreadsheetToken 和 conditionFormatRules 批量创建条件格式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-format/create-condition-formats
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量创建条件格式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionFormatParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 条件格式规则列表
    #[serde(rename = "conditionFormatRules")]
    pub condition_format_rules: Vec<ConditionFormatRule>,
}

/// 条件格式规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatRule {
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
    /// 条件内容
    #[serde(rename = "condition")]
    pub condition: ConditionContent,
    /// 格式设置
    #[serde(rename = "format")]
    pub format: FormatSettings,
}

/// 条件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionContent {
    /// 比较类型
    #[serde(rename = "type")]
    pub condition_type: String,
    /// 条件值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// 格式设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatSettings {
    /// 背景色
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// 前景色
    #[serde(rename = "foregroundColor", skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    /// 字体加粗
    #[serde(rename = "bold", skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 字体斜体
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 字体删除线
    #[serde(rename = "strikethrough", skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
}

/// 批量创建条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionFormatResponse {
    /// 创建结果
    pub data: Option<CreateConditionFormatResult>,
}

/// 批量创建条件格式结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionFormatResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 创建的条件格式规则列表
    #[serde(
        rename = "conditionFormatRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_format_rules: Option<Vec<ConditionFormatRuleResult>>,
}

/// 条件格式规则结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatRuleResult {
    /// 条件格式规则ID
    #[serde(rename = "conditionFormatRuleId")]
    pub condition_format_rule_id: i64,
    /// 条件格式规则信息
    #[serde(rename = "conditionFormatRule")]
    pub condition_format_rule: ConditionFormatRule,
}

impl ApiResponseTrait for CreateConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建条件格式请求
pub struct CreateConditionFormatRequest {
    config: Config,
}

impl CreateConditionFormatRequest {
    /// 创建批量创建条件格式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-format/create-condition-formats
    pub async fn execute(
        self,
        params: CreateConditionFormatParams,
    ) -> SDKResult<CreateConditionFormatResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint =
            CcmSheetApiOld::ConditionFormatsBatchCreate(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateConditionFormatResponse> = ApiRequest::post(
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
