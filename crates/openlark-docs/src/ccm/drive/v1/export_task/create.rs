use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 创建导出任务
///
/// 创建一个导出任务，将云文档导出为文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/create
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 创建导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskRequest {
    /// 文件token
    pub token: String,
    /// 导出文件类型
    pub r#type: String,
    /// 子类型
    pub sub_type: Option<String>,
}

impl CreateExportTaskRequest {
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
            sub_type: None,
        }
    }
}

/// 创建导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskResponse {
    /// 任务ticket
    pub ticket: Option<String>,
}

impl ApiResponseTrait for CreateExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建导出任务
pub async fn create_export_task(
    request: CreateExportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateExportTaskResponse>> {
    let api_endpoint = DriveApi::CreateExportTask;

    let mut api_request: ApiRequest<CreateExportTaskResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}