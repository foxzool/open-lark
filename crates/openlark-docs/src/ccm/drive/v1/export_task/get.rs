use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取导出任务状态
///
/// 获取导出任务的执行状态。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/get
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

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

    pub async fn execute(self) -> SDKResult<GetExportTaskResponse> {
        if self.ticket.is_empty() {
            return Err(openlark_core::error::validation_error(
                "ticket",
                "ticket 不能为空",
            ));
        }
        let token_len = self.token.as_bytes().len();
        if token_len == 0 || token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 长度必须在 1~27 字节之间",
            ));
        }

        let api_endpoint = DriveApi::GetExportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetExportTaskResponse>::get(&api_endpoint.to_url())
            .query("token", &self.token);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "查询导出任务结果")
    }
}

/// 获取导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskResponse {
    /// 导出任务结果
    pub result: ExportTaskResult,
}

/// 导出任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskResult {
    /// 导出的文件的扩展名
    pub file_extension: String,
    /// 要导出的云文档类型
    pub r#type: String,
    /// 导出的文件名称
    pub file_name: Option<String>,
    /// 导出的文件 token（用于下载导出文件）
    pub file_token: Option<String>,
    /// 导出文件大小（字节）
    pub file_size: Option<i32>,
    /// 导出任务失败原因
    pub job_error_msg: Option<String>,
    /// 导出任务状态
    pub job_status: Option<i32>,
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
