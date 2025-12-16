use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取导出任务状态
///
/// 获取导出任务的执行状态。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/get
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskRequest {
    /// 任务ticket
    pub ticket: String,
    /// 文件token
    pub token: String,
}

impl GetExportTaskRequest {
    pub fn new(ticket: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            ticket: ticket.into(),
            token: token.into(),
        }
    }
}

/// 获取导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskResponse {
    /// 任务状态
    pub job_status: Option<i32>,
    /// 错误信息
    pub job_error_msg: Option<String>,
    /// 下载token
    pub file_token: Option<String>,
    /// 文件大小
    pub file_size: Option<i64>,
}

impl ApiResponseTrait for GetExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取导出任务状态
pub async fn get_export_task(
    request: GetExportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetExportTaskResponse>> {
    let api_endpoint = DriveApi::GetExportTask(request.ticket.clone());

    let mut api_request: ApiRequest<GetExportTaskResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query("token", &request.token);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}