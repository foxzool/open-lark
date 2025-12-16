use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 创建导入任务
///
/// 创建一个导入任务，将文件导入到云空间。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 创建导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    /// 文件扩展名
    pub file_extension: String,
    /// 文件token
    pub file_token: String,
    /// 目标类型
    pub r#type: String,
    /// 文件名
    pub file_name: Option<String>,
    /// 目录token
    pub point: Option<Point>,
}

/// 目标目录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// 挂载类型
    pub mount_type: i32,
    /// 挂载key
    pub mount_key: String,
}

impl CreateImportTaskRequest {
    pub fn new(file_extension: impl Into<String>, file_token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            file_extension: file_extension.into(),
            file_token: file_token.into(),
            r#type: r#type.into(),
            file_name: None,
            point: None,
        }
    }
}

/// 创建导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskResponse {
    /// 任务ticket
    pub ticket: Option<String>,
}

impl ApiResponseTrait for CreateImportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建导入任务
pub async fn create_import_task(
    request: CreateImportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateImportTaskResponse>> {
    let api_endpoint = DriveApi::CreateImportTask;

    let mut api_request: ApiRequest<CreateImportTaskResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}