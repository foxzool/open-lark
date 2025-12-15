/// 创建导入任务
///
/// 创建文件导入任务，支持从云空间或上传任务导入文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    /// 父目录的token
    pub parent_type: String,
    /// 父目录的token
    pub parent_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件token（用于从云空间导入）
    pub file_token: Option<String>,
    /// 上传任务token（用于从上传任务导入）
    pub upload_task_token: Option<String>,
    /// 导入选项
    pub options: Option<ImportOptions>,
}

impl CreateImportTaskRequest {
    /// 创建导入任务请求
    ///
    /// # 参数
    /// * `parent_type` - 父目录类型
    /// * `parent_token` - 父目录token
    /// * `file_name` - 文件名
    /// * `file_type` - 文件类型
    pub fn new(
        parent_type: impl Into<String>,
        parent_token: impl Into<String>,
        file_name: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            parent_type: parent_type.into(),
            parent_token: parent_token.into(),
            file_name: file_name.into(),
            r#type: file_type.into(),
            file_token: None,
            upload_task_token: None,
            options: None,
        }
    }

    /// 设置文件token（用于从云空间导入）
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置上传任务token（用于从上传任务导入）
    pub fn upload_task_token(mut self, upload_task_token: impl Into<String>) -> Self {
        self.upload_task_token = Some(upload_task_token.into());
        self
    }

    /// 设置导入选项
    pub fn options(mut self, options: ImportOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// 导入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    /// 是否覆盖同名文件
    pub overwrite: Option<bool>,
    /// 导入模式
    pub mode: Option<String>,
    /// 目标类型
    pub target_type: Option<String>,
}

impl ImportOptions {
    /// 创建导入选项
    pub fn new() -> Self {
        Self {
            overwrite: None,
            mode: None,
            target_type: None,
        }
    }

    /// 设置是否覆盖同名文件
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    /// 设置导入模式
    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// 设置目标类型
    pub fn target_type(mut self, target_type: impl Into<String>) -> Self {
        self.target_type = Some(target_type.into());
        self
    }
}

/// 创建导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskResponse {
    /// 导入任务信息
    pub task: ImportTaskInfo,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for CreateImportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskInfo {
    /// 任务token
    pub task_token: String,
    /// 任务状态
    pub status: String,
    /// 文件token
    pub file_token: Option<String>,
    /// 导入类型
    pub r#type: String,
    /// 文件名
    pub file_name: String,
    /// 创建时间
    pub created_at: String,
}

/// 创建导入任务
///
/// 创建文件导入任务，支持从云空间或上传任务导入文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks
pub async fn create_import_task(
    request: CreateImportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateImportTaskResponse>> {
    // 构建请求体
    let mut body = json!({
        "parentType": request.parent_type,
        "parentToken": request.parent_token,
        "fileName": request.file_name,
        "type": request.r#type
    });

    if let Some(file_token) = &request.file_token {
        body["fileToken"] = json!(file_token);
    }
    if let Some(upload_task_token) = &request.upload_task_token {
        body["uploadTaskToken"] = json!(upload_task_token);
    }
    if let Some(options) = &request.options {
        body["options"] = json!(options);
    }

    // 创建API请求
    let url = DriveApi::CreateImportTask.to_url();
    let mut api_request: ApiRequest<CreateImportTaskResponse> =
        ApiRequest::post(&url)
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_import_task_request_builder() {
        let options = ImportOptions::new()
            .overwrite(false)
            .mode("create")
            .target_type("doc");

        let request = CreateImportTaskRequest::new(
            "explorer",
            "parent_token",
            "imported_doc.docx",
            "docx"
        )
        .file_token("file_token")
        .options(options);

        assert_eq!(request.parent_type, "explorer");
        assert_eq!(request.parent_token, "parent_token");
        assert_eq!(request.file_name, "imported_doc.docx");
        assert_eq!(request.file_token, Some("file_token".to_string()));
    }

    #[test]
    fn test_import_options_builder() {
        let options = ImportOptions::new()
            .overwrite(true)
            .mode("update")
            .target_type("sheet");

        assert_eq!(options.overwrite, Some(true));
        assert_eq!(options.mode, Some("update".to_string()));
        assert_eq!(options.target_type, Some("sheet".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateImportTaskResponse::data_format(), ResponseFormat::Data);
    }
}