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
    #[serde(skip)]
    config: Config,
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
    pub fn new(
        config: Config,
        file_extension: impl Into<String>,
        file_token: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_extension: file_extension.into(),
            file_token: file_token.into(),
            r#type: r#type.into(),
            file_name: None,
            point: None,
        }
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn point(mut self, point: Point) -> Self {
        self.point = Some(point);
        self
    }

    pub async fn execute(self) -> SDKResult<Response<CreateImportTaskResponse>> {
        let api_endpoint = DriveApi::CreateImportTask;

        let api_request = ApiRequest::<CreateImportTaskResponse>::post(&api_endpoint.to_url())
            .body(serde_json::json!({
                "file_extension": self.file_extension,
                "file_token": self.file_token,
                "type": self.r#type,
                "file_name": self.file_name,
                "point": self.point
            }));

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_import_task_request_builder() {
        let config = Config::default();
        let request = CreateImportTaskRequest::new(config, "pdf", "file_token", "sheet")
            .file_name("test_file");

        assert_eq!(request.file_extension, "pdf");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "sheet");
        assert_eq!(request.file_name, Some("test_file".to_string()));
    }

    #[test]
    fn test_point_structure() {
        let point = Point {
            mount_type: 1,
            mount_key: "mount_key".to_string(),
        };

        assert_eq!(point.mount_type, 1);
        assert_eq!(point.mount_key, "mount_key");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateImportTaskResponse::data_format(), ResponseFormat::Data);
    }
}