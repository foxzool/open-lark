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
    #[serde(skip)]
    config: Config,
    /// 任务ticket
    pub ticket: String,
    /// 文件token
    pub token: String,
}

impl GetExportTaskRequest {
    pub fn new(config: Config, ticket: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            config,
            ticket: ticket.into(),
            token: token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<GetExportTaskResponse>> {
        let api_endpoint = DriveApi::GetExportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetExportTaskResponse>::get(&api_endpoint.to_url())
            .query("token", &self.token);

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_export_task_request_builder() {
        let config = Config::default();
        let request = GetExportTaskRequest::new(config, "ticket", "token");
        assert_eq!(request.ticket, "ticket");
        assert_eq!(request.token, "token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetExportTaskResponse::data_format(), ResponseFormat::Data);
    }
}
