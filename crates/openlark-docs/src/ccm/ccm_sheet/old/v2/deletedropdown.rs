/// 删除数据验证规则
///
/// 根据 spreadsheetToken 和 dataValidationRuleIds 批量删除数据验证规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/delete-data-validations
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 删除数据验证规则请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDropdownParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据验证规则ID列表
    #[serde(rename = "dataValidationRuleIds")]
    pub data_validation_rule_ids: Vec<i64>,
}

/// 删除数据验证规则响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDropdownResponse {
    /// 删除结果
    pub data: Option<DeleteDropdownResult>,
}

/// 删除数据验证规则结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDropdownResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 删除的数据验证规则ID列表
    #[serde(
        rename = "deletedDataValidationRuleIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_data_validation_rule_ids: Option<Vec<i64>>,
}

impl ApiResponseTrait for DeleteDropdownResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除数据验证规则请求
pub struct DeleteDropdownRequest {
    config: Config,
}

impl DeleteDropdownRequest {
    /// 创建删除数据验证规则请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/delete-data-validations
    pub async fn execute(self, params: DeleteDropdownParams) -> SDKResult<DeleteDropdownResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::DataValidationDelete(
            params.spreadsheet_token.clone(),
            "".to_string(), // 将在body中传递具体ID列表
        );

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteDropdownResponse> = ApiRequest::post(
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
