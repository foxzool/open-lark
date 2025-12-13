/// 操作工作表
///
/// 根据 spreadsheetToken 更新工作表属性。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 操作工作表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateSheetsParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作请求
    pub requests: Vec<SheetOperationRequest>,
}

/// 工作表操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetOperationRequest {
    /// 操作类型
    #[serde(rename = "operateType")]
    pub operate_type: String,
    /// 工作表属性
    pub properties: Option<serde_json::Value>,
    /// 其他字段
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// 操作工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateSheetsResponse {
    /// 操作结果
    pub data: Option<OperateResult>,
}

/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作结果列表
    #[serde(rename = "operateResults")]
    pub operate_results: Vec<OperationResult>,
}

/// 单个操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

impl ApiResponseTrait for OperateSheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 操作工作表请求
pub struct OperateSheetsRequest {
    config: Config,
}

impl OperateSheetsRequest {
    /// 创建操作工作表请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets
    pub async fn execute(self, params: OperateSheetsParams) -> SDKResult<OperateSheetsResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::OperateSheets(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<OperateSheetsResponse> = ApiRequest::post(
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
