/// 批量删除条件格式
///
/// 根据 spreadsheetToken 和条件格式ID列表批量删除条件格式。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/conditional-format/batch_delete_condition_format
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量删除条件格式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionFormatParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 条件格式ID列表
    #[serde(rename = "conditionFormatIdList")]
    pub condition_format_id_list: Vec<String>,
}

/// 批量删除条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionFormatResponse {
    /// 操作结果
    pub data: Option<DeleteConditionFormatResult>,
}

/// 删除条件格式结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionFormatResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 删除结果
    pub delete_info: Option<Vec<DeleteInfo>>,
}

/// 删除信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInfo {
    /// 条件格式ID
    #[serde(rename = "conditionFormatId")]
    pub condition_format_id: String,
    /// 删除状态
    pub status: String,
    /// 错误信息（如果删除失败）
    pub error: Option<String>,
}

impl ApiResponseTrait for DeleteConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除条件格式请求
pub struct DeleteConditionFormatRequest {
    config: Config,
}

impl DeleteConditionFormatRequest {
    /// 创建批量删除条件格式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/conditional-format/batch_delete_condition_format
    pub async fn execute(
        self,
        params: DeleteConditionFormatParams,
    ) -> SDKResult<DeleteConditionFormatResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.condition_format_id_list, "条件格式ID列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint =
            CcmSheetApiOld::ConditionFormatsBatchDelete(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteConditionFormatResponse> = ApiRequest::post(
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
