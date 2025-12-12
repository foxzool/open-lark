//! 批量更新条件格式
//!
//! 根据 spreadsheetToken 批量更新条件格式规则。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-formatting/batch-update-conditional-formats

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量更新条件格式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionFormatParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 条件格式更新请求列表
    pub updates: Vec<ConditionFormatUpdate>,
}

/// 条件格式更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatUpdate {
    /// 条件格式ID
    #[serde(rename = "conditionalFormatId")]
    pub conditional_format_id: String,
    /// 条件格式规则
    pub rule: ConditionFormatRule,
}

/// 条件格式规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatRule {
    /// 规则类型
    #[serde(rename = "type")]
    pub rule_type: String,
    /// 条件
    pub condition: serde_json::Value,
    /// 格式样式
    pub format: Option<serde_json::Value>,
}

/// 批量更新条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionFormatResponse {
    /// 更新结果
    pub data: Option<UpdateConditionFormatResult>,
}

/// 更新条件格式结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionFormatResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 是否成功
    pub success: bool,
    /// 更新的规则列表
    #[serde(rename = "conditionalFormats")]
    pub conditional_formats: Option<Vec<ConditionFormatInfo>>,
}

/// 条件格式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatInfo {
    /// 条件格式ID
    #[serde(rename = "conditionalFormatId")]
    pub conditional_format_id: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 规则
    pub rule: ConditionFormatRule,
}

impl ApiResponseTrait for UpdateConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新条件格式请求
pub struct UpdateConditionFormatRequest {
    config: Config,
}

impl UpdateConditionFormatRequest {
    /// 创建批量更新条件格式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-formatting/batch-update-conditional-formats
    pub async fn execute(
        self,
        params: UpdateConditionFormatParams,
    ) -> SDKResult<UpdateConditionFormatResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchUpdate(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<UpdateConditionFormatResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}