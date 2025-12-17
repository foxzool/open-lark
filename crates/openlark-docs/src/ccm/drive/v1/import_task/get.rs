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
    #[serde(skip)]
    config: Config,
    /// 任务ticket
    pub ticket: String,
}

impl GetImportTaskRequest {
    pub fn new(config: Config, ticket: impl Into<String>) -> Self {
        Self {
            config,
            ticket: ticket.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<GetImportTaskResponse>> {
        let api_endpoint = DriveApi::GetImportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetImportTaskResponse>::get(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_import_task_request_builder() {
        let config = Config::default();
        let request = GetImportTaskRequest::new(config, "ticket");
        assert_eq!(request.ticket, "ticket");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetImportTaskResponse::data_format(), ResponseFormat::Data);
    }
}
