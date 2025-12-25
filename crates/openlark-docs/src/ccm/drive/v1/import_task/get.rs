use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取导入任务状态
///
/// 获取导入任务的执行状态。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/get
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

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

    pub async fn execute(self) -> SDKResult<GetImportTaskResponse> {
        if self.ticket.is_empty() {
            return Err(openlark_core::error::validation_error(
                "ticket",
                "ticket 不能为空",
            ));
        }

        let api_endpoint = DriveApi::GetImportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetImportTaskResponse>::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "查询导入任务结果")
    }
}

/// 获取导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskResponse {
    /// 导入结果
    pub result: ImportTaskResult,
}

/// 导入任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskResult {
    /// 导入任务 ID
    pub ticket: String,
    /// 导入的在线云文档类型（docx/sheet/bitable）
    pub r#type: String,
    /// 任务状态
    pub job_status: Option<i32>,
    /// 任务失败原因
    pub job_error_msg: Option<String>,
    /// 导入云文档的 token
    pub token: Option<String>,
    /// 导入云文档的 URL
    pub url: Option<String>,
    /// 导入成功的额外提示
    #[serde(default)]
    pub extra: Vec<String>,
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
