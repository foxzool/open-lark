/// 获取条件格式
///
/// 根据 spreadsheetToken 获取条件格式规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-formatting/get-conditional-formats
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取条件格式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionFormatParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID，可选。不填则获取所有工作表的条件格式
    #[serde(rename = "sheetId", skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
}

/// 获取条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionFormatResponse {
    /// 条件格式信息
    pub data: Option<ConditionFormatResult>,
}

/// 条件格式结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFormatResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 条件格式列表
    #[serde(rename = "conditionalFormats")]
    pub conditional_formats: Vec<ConditionFormatInfo>,
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
    /// 规则名称
    pub name: String,
    /// 规则
    pub rule: ConditionFormatRule,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: i64,
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

impl ApiResponseTrait for GetConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取条件格式请求
pub struct GetConditionFormatRequest {
    config: Config,
}

impl GetConditionFormatRequest {
    /// 创建获取条件格式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditional-formatting/get-conditional-formats
    pub async fn execute(
        self,
        params: GetConditionFormatParams,
    ) -> SDKResult<GetConditionFormatResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ConditionFormats(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetConditionFormatResponse> = ApiRequest::post(
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
