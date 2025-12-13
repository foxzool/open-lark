/// 获取保护范围
///
/// 根据 spreadsheetToken 获取表格的保护范围列表。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protection/get-protected-ranges
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取保护范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProtectedRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID列表
    #[serde(rename = "sheetIds", skip_serializing_if = "Option::is_none")]
    pub sheet_ids: Option<Vec<i64>>,
}

/// 获取保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProtectedRangeResponse {
    /// 获取结果
    pub data: Option<GetProtectedRangeResult>,
}

/// 获取保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProtectedRangeResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 保护范围列表
    #[serde(rename = "protectedRanges", skip_serializing_if = "Option::is_none")]
    pub protected_ranges: Option<Vec<ProtectedRangeResult>>,
}

/// 保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeResult {
    /// 保护范围ID
    #[serde(rename = "protectedRangeId")]
    pub protected_range_id: i64,
    /// 保护范围信息
    #[serde(rename = "protectedRange")]
    pub protected_range: ProtectedRange,
}

/// 保护范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRange {
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
    /// 保护范围名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 保护范围描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否警告用户
    #[serde(rename = "warningOnly", skip_serializing_if = "Option::is_none")]
    pub warning_only: Option<bool>,
    /// 编辑者列表
    #[serde(rename = "editors", skip_serializing_if = "Option::is_none")]
    pub editors: Option<Editors>,
}

/// 编辑者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editors {
    /// 用户列表
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// 邮箱列表
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// 群组列表
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// 域名列表
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
}

impl ApiResponseTrait for GetProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取保护范围请求
pub struct GetProtectedRangeRequest {
    config: Config,
}

impl GetProtectedRangeRequest {
    /// 创建获取保护范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protection/get-protected-ranges
    pub async fn execute(
        self,
        params: GetProtectedRangeParams,
    ) -> SDKResult<GetProtectedRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchGet(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetProtectedRangeResponse> = ApiRequest::post(
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
