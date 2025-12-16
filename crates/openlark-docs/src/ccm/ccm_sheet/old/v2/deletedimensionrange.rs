/// 删除行列
///
/// 根据 spreadsheetToken 和 deleteDimensionRequest 删除指定工作表中的行或列。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheets/delete-dimension-range
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 删除行列请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 删除维度请求
    pub delete_dimension: DeleteDimensionRequest,
}

/// 删除维度请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRequest {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 维度类型：ROWS（行）或 COLUMNS（列）
    pub dimension: String,
    /// 起始索引（从0开始）
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引（不包含）
    #[serde(rename = "endIndex")]
    pub end_index: i32,
}

/// 删除行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeResponse {
    /// 操作结果
    pub data: Option<DeleteDimensionResult>,
}

/// 删除维度操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作是否成功
    pub success: bool,
    /// 删除的行列范围
    #[serde(rename = "deletedDimensionRange")]
    pub deleted_dimension_range: Option<DimensionRange>,
}

/// 维度范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionRange {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 维度类型
    pub dimension: String,
    /// 起始索引
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引
    #[serde(rename = "endIndex")]
    pub end_index: i32,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列请求
pub struct DeleteDimensionRangeRequest {
    config: Config,
}

impl DeleteDimensionRangeRequest {
    /// 创建删除行列请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheets/delete-dimension-range
    pub async fn execute(
        self,
        params: DeleteDimensionRangeParams,
    ) -> SDKResult<DeleteDimensionRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::DimensionRangeDelete(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteDimensionRangeResponse> = ApiRequest::post(
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
