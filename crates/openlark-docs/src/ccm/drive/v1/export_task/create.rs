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
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 导出文件类型
    pub r#type: String,
    /// 子类型
    pub sub_type: Option<String>,
}

impl CreateExportTaskRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
            sub_type: None,
        }
    }

    pub fn sub_type(mut self, sub_type: impl Into<String>) -> Self {
        self.sub_type = Some(sub_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<CreateExportTaskResponse>> {
        let api_endpoint = DriveApi::CreateExportTask;

        let api_request = ApiRequest::<CreateExportTaskResponse>::post(&api_endpoint.to_url())
            .body(serde_json::json!({
                "token": self.token,
                "type": self.r#type,
                "sub_type": self.sub_type
            }));

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_export_task_request_builder() {
        let config = Config::default();
        let request = CreateExportTaskRequest::new(config, "file_token", "pdf")
            .sub_type("param");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "pdf");
        assert_eq!(request.sub_type, Some("param".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateExportTaskResponse::data_format(), ResponseFormat::Data);
    }
}