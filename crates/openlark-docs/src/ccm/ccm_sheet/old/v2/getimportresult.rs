/// 获取导入结果
///
/// 根据任务ID查询电子表格导入的进度和结果。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/import/get_import_result
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取导入结果请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportResultParams {
    /// 任务ID
    #[serde(rename = "taskId")]
    pub task_id: String,
}

/// 导入状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImportStatus {
    /// 处理中
    Processing,
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 导入进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportProgress {
    /// 进度百分比 (0-100)
    pub percentage: i32,
    /// 当前步骤
    pub current_step: Option<String>,
    /// 预计剩余时间（秒）
    pub estimated_time_remaining: Option<i32>,
}

/// 获取导入结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportResultResponse {
    /// 操作结果
    pub data: Option<GetImportResultData>,
}

/// 获取导入结果数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportResultData {
    /// 任务ID
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// 导入状态
    pub status: ImportStatus,
    /// 进度信息（处理中时返回）
    pub progress: Option<ImportProgress>,
    /// 电子表格的 token（成功时返回）
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: Option<String>,
    /// 错误信息（失败时返回）
    pub error: Option<String>,
    /// 导入统计信息（成功时返回）
    pub statistics: Option<ImportStatistics>,
}

/// 导入统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatistics {
    /// 总行数
    #[serde(rename = "totalRows")]
    pub total_rows: i32,
    /// 成功导入的行数
    #[serde(rename = "importedRows")]
    pub imported_rows: i32,
    /// 失败的行数
    #[serde(rename = "failedRows")]
    pub failed_rows: i32,
    /// 总列数
    #[serde(rename = "totalColumns")]
    pub total_columns: i32,
    /// 导入的工作表数量
    #[serde(rename = "importedSheets")]
    pub imported_sheets: i32,
}

impl ApiResponseTrait for GetImportResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取导入结果请求
pub struct GetImportResultRequest {
    config: Config,
}

impl GetImportResultRequest {
    /// 创建获取导入结果请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/import/get_import_result
    pub async fn execute(
        self,
        params: GetImportResultParams,
    ) -> SDKResult<GetImportResultResponse> {
        // 验证必填字段
        validate_required!(params.task_id, "任务ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ImportResult;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetImportResultResponse> = ApiRequest::post(
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
