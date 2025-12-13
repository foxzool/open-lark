/// 导入电子表格
///
/// 上传文件并创建新的电子表格，支持多种格式的文件导入。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/import/import_spreadsheet
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validation_error, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 导入电子表格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpreadsheetParams {
    /// 上传的文件信息
    pub file: FileInfo,
    /// 导入选项
    pub import_options: Option<ImportOptions>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件名称
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// 文件大小（字节）
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// 文件类型
    #[serde(rename = "fileType")]
    pub file_type: String,
    /// 文件内容（Base64编码）
    #[serde(rename = "fileContent")]
    pub file_content: String,
    /// 工作表名称（可选）
    #[serde(rename = "sheetName")]
    pub sheet_name: Option<String>,
}

/// 导入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    /// 是否包含标题行
    #[serde(rename = "hasTitleRow")]
    pub has_title_row: Option<bool>,
    /// 是否覆盖现有电子表格
    #[serde(rename = "overwrite")]
    pub overwrite: Option<bool>,
    /// 目标工作表ID（用于覆盖现有工作表）
    #[serde(rename = "targetSheetId")]
    pub target_sheet_id: Option<String>,
    /// 导入工作表名称
    #[serde(rename = "importSheetName")]
    pub import_sheet_name: Option<String>,
}

/// 导入电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpreadsheetResponse {
    /// 操作结果
    pub data: Option<ImportSpreadsheetResult>,
}

/// 导入电子表格结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpreadsheetResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 任务ID
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// 导入状态
    pub status: String,
    /// 错误信息（如果导入失败）
    pub error: Option<String>,
}

impl ApiResponseTrait for ImportSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入电子表格请求
pub struct ImportSpreadsheetRequest {
    config: Config,
}

impl ImportSpreadsheetRequest {
    /// 创建导入电子表格请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/import/import_spreadsheet
    pub async fn execute(
        self,
        params: ImportSpreadsheetParams,
    ) -> SDKResult<ImportSpreadsheetResponse> {
        // 验证必填字段
        validate_required!(params.file.file_name, "文件名称不能为空");
        if params.file.file_size <= 0 {
            return Err(validation_error("file_size", "文件大小必须大于0"));
        }
        validate_required!(params.file.file_type, "文件类型不能为空");
        validate_required!(params.file.file_content, "文件内容不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::Import;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ImportSpreadsheetResponse> = ApiRequest::post(
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
