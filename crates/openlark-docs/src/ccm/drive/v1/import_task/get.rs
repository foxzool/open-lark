use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取导入任务状态
///
/// 获取导入任务的执行状态。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskRequest {
    /// 任务ticket
    pub ticket: String,
}

impl GetImportTaskRequest {
    pub fn new(ticket: impl Into<String>) -> Self {
        Self {
            ticket: ticket.into(),
        }
    }
}

/// 获取导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskResponse {
    /// 任务状态
    pub job_status: Option<i32>,
    /// 错误信息
    pub job_error_msg: Option<String>,
    /// 目标token
    pub token: Option<String>,
    /// 目标URL
    pub url: Option<String>,
}

impl ApiResponseTrait for GetImportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取导入任务状态
pub async fn get_import_task(
    request: GetImportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetImportTaskResponse>> {
    let api_endpoint = DriveApi::GetImportTask(request.ticket.clone());

    let mut api_request: ApiRequest<GetImportTaskResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}