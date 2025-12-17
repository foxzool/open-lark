use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 移动文件或文件夹
///
/// 将文件或者文件夹移动到用户云空间的其他位置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/move
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 移动文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 目标文件夹token
    pub parent_folder_token: String,
    /// 新文件名称，可选，不传则使用原文件名
    pub name: Option<String>,
}

impl MoveFileRequest {
    /// 创建移动文件请求
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

    pub async fn execute(self) -> SDKResult<Response<MoveFileResponse>> {
        let api_endpoint = DriveApi::MoveFile(self.file_token.clone());
        let mut request = ApiRequest::<MoveFileResponse>::post(&api_endpoint.to_url());
        
        let mut body = serde_json::Map::new();
        body.insert("parent_folder_token".to_string(), serde_json::Value::String(self.parent_folder_token.clone()));
        if let Some(name) = &self.name {
            body.insert("name".to_string(), serde_json::Value::String(name.clone()));
        }

        request = request.json_body(&body);

        Transport::request(request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file_request_builder() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.parent_folder_token, "folder_token");
        assert!(request.name.is_none());
    }

    #[test]
    fn test_move_file_request_with_name() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "folder_token")
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
            update_time: 1640995300,
        };

        assert_eq!(file_data.file_token, "file_token");
        assert_eq!(file_data.name, "test_file.txt");
        assert_eq!(file_data.r#type, "file");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            MoveFileResponse::data_format(),
            ResponseFormat::Data
        );
    }
}