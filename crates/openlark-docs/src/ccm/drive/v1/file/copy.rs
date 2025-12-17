use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 复制文件
///
/// 将文件复制到用户云空间的其他文件夹中。不支持复制文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 复制文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 目标文件夹token
    pub parent_folder_token: String,
    /// 新文件名称，可选，不传则使用原文件名
    pub name: Option<String>,
}

impl CopyFileRequest {
    /// 创建复制文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 文件token
    /// * `parent_folder_token` - 目标文件夹token
    pub fn new(config: Config, file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            parent_folder_token: parent_folder_token.into(),
            name: None,
        }
    }

    /// 设置文件名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<CopyFileResponse>> {
        let api_endpoint = DriveApi::CopyFile(self.file_token.clone());
        let mut request = ApiRequest::<CopyFileResponse>::post(&api_endpoint.to_url());
        
        let mut body = serde_json::Map::new();
        body.insert("parent_folder_token".to_string(), serde_json::Value::String(self.parent_folder_token.clone()));
        if let Some(name) = &self.name {
            body.insert("name".to_string(), serde_json::Value::String(name.clone()));
        }

        request = request.json_body(&body);

        Transport::request(request, &self.config, None).await
    }
}

/// 复制文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// 文件夹token
    pub folder_token: String,
    /// 修正后的文件名
    pub revision: i32,
    /// 包含文件详情
    pub file: FileData,
}

/// 文件详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    /// 文件token
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 创建时间
    pub create_time: i64,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_request_builder() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "file_token", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.parent_folder_token, "folder_token");
        assert!(request.name.is_none());
    }

    #[test]
    fn test_copy_file_request_with_name() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "file_token", "folder_token")
            .name("new_file_name");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.parent_folder_token, "folder_token");
        assert_eq!(request.name.unwrap(), "new_file_name");
    }

    #[test]
    fn test_file_data_structure() {
        let file_data = FileData {
            file_token: "file_token".to_string(),
            name: "test_file.txt".to_string(),
            r#type: "file".to_string(),
            create_time: 1640995200,
        };

        assert_eq!(file_data.file_token, "file_token");
        assert_eq!(file_data.name, "test_file.txt");
        assert_eq!(file_data.r#type, "file");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CopyFileResponse::data_format(),
            ResponseFormat::Data
        );
    }
}