/// 删除保护范围
///
/// 根据 spreadsheetToken 和 protectedRangeIds 批量删除保护范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protection/delete-protected-ranges
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 删除保护范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 保护范围ID列表
    #[serde(rename = "protectedRangeIds")]
    pub protected_range_ids: Vec<i64>,
}

/// 删除保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangeResponse {
    /// 删除结果
    pub data: Option<DeleteProtectedRangeResult>,
}

/// 删除保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangeResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 删除的保护范围ID列表
    #[serde(
        rename = "deletedProtectedRangeIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted_protected_range_ids: Option<Vec<i64>>,
}

impl ApiResponseTrait for DeleteProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除保护范围请求
pub struct DeleteProtectedRangeRequest {
    config: Config,
}

impl DeleteProtectedRangeRequest {
    /// 创建删除保护范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protection/delete-protected-ranges
    pub async fn execute(
        self,
        params: DeleteProtectedRangeParams,
    ) -> SDKResult<DeleteProtectedRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchDel(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<DeleteProtectedRangeResponse> = ApiRequest::post(
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
