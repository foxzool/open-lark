/// 创建文件快捷方式
///
/// 创建文件快捷方式，用于访问云空间的文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create_shortcut

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;


/// 创建文件快捷方式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutResponse {
    /// 快捷方式信息
    pub data: Option<FileShortcutData>,
}

/// 文件快捷方式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileShortcutData {
    /// 文件token
    pub file_token: String,
    /// 快捷方式名称
    pub name: String,
    /// 快捷方式类型
    pub shortcut_type: String,
    /// 创建时间
    pub create_time: String,
}

impl ApiResponseTrait for CreateFileShortcutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文件快捷方式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutRequest {
    #[serde(skip)]
    config: Config,
    /// 快捷方式名称
    pub name: String,
    /// 快捷方式类型
    pub shortcut_type: String,
    /// 目标文件token
    pub file_token: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

impl CreateFileShortcutRequest {
    /// 创建新的请求实例
    pub fn new(
        config: Config,
        name: impl Into<String>,
        shortcut_type: impl Into<String>,
        file_token: impl Into<String>,
        parent_folder_token: impl Into<String>
    ) -> Self {
        Self { 
            config,
            name: name.into(),
            shortcut_type: shortcut_type.into(),
            file_token: file_token.into(),
            parent_folder_token: parent_folder_token.into(),
        }
    }

    /// 执行创建文件快捷方式操作
    pub async fn execute(self) -> SDKResult<Response<CreateFileShortcutResponse>> {
        let api_endpoint = DriveApi::CreateShortcut;
        let request = ApiRequest::<CreateFileShortcutResponse>::post(&api_endpoint.to_url())
            .json_body(&self);

        Transport::request(request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_shortcut_request() {
        let config = Config::default();
        let request = CreateFileShortcutRequest::new(
            config,
            "快捷方式名称",
            "document",
            "file_token_123",
            "folder_token_456"
        );

        assert_eq!(request.name, "快捷方式名称");
        assert_eq!(request.shortcut_type, "document");
        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.parent_folder_token, "folder_token_456");
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CreateFileShortcutResponse::data_format(), ResponseFormat::Data);
    }
}