use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 创建导入任务
///
/// 创建导入任务，支持导入为新版文档、电子表格、多维表格以及旧版文档（异步接口）。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 目录token
    pub point: Point,
}

/// 目标目录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// 挂载类型
    pub mount_type: i32,
    /// 挂载key
    pub mount_key: String,
}

impl Point {
    /// 创建挂载点（挂载到云空间，mount_type 固定为 1）
    pub fn new(mount_key: impl Into<String>) -> Self {
        Self {
            mount_type: 1,
            mount_key: mount_key.into(),
        }
    }
}

impl CreateImportTaskRequest {
    pub fn new(
        config: Config,
        file_extension: impl Into<String>,
        file_token: impl Into<String>,
        r#type: impl Into<String>,
        point: Point,
    ) -> Self {
        Self {
            config,
            file_extension: file_extension.into(),
            file_token: file_token.into(),
            r#type: r#type.into(),
            file_name: None,
            point,
        }
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateImportTaskResponse> {
        if self.file_extension.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_extension",
                "file_extension 不能为空",
            ));
        }
        let file_token_len = self.file_token.as_bytes().len();
        if file_token_len == 0 || file_token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 长度必须在 1~27 字节之间",
            ));
        }
        match self.r#type.as_str() {
            "docx" | "sheet" | "bitable" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 docx/sheet/bitable",
                ))
            }
        }
        if self.point.mount_type != 1 {
            return Err(openlark_core::error::validation_error(
                "point.mount_type",
                "point.mount_type 仅支持固定值 1（挂载到云空间）",
            ));
        }

        let api_endpoint = DriveApi::CreateImportTask;

        let api_request: ApiRequest<CreateImportTaskResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self, "创建导入任务")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建导入任务")
    }
}

/// 创建导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskResponse {
    /// 任务ticket
    pub ticket: String,
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
        let request = CreateImportTaskRequest::new(
            config,
            "pdf",
            "file_token",
            "sheet",
            Point::new("AbqrfuRTjlJEIJduwDwcnIabcef"),
        )
        .file_name("test_file");

        assert_eq!(request.file_extension, "pdf");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "sheet");
        assert_eq!(request.file_name, Some("test_file".to_string()));
    }

    #[test]
    fn test_point_structure() {
        let point = Point::new("mount_key");

        assert_eq!(point.mount_type, 1);
        assert_eq!(point.mount_key, "mount_key");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateImportTaskResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
