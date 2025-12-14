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
    /// * `file_token` - 文件token
    /// * `parent_folder_token` - 目标文件夹token
    pub fn new(file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
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
}

/// 移动文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFileResponse {
    /// 移动后的文件信息
    pub data: Option<FileData>,
}

/// 文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
}

impl ApiResponseTrait for MoveFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动文件或文件夹
///
/// 将文件或者文件夹移动到用户云空间的其他位置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/move
pub async fn move_file(
    request: MoveFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<MoveFileResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::MoveFile(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<MoveFileResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "parent_folder_token": request.parent_folder_token,
            "name": request.name
        }));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file_request_builder() {
        let request = MoveFileRequest::new("file_token", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.parent_folder_token, "folder_token");
        assert!(request.name.is_none());
    }

    #[test]
    fn test_move_file_request_with_name() {
        let request = MoveFileRequest::new("file_token", "folder_token")
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