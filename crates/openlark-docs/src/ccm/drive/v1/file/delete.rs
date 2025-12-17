/// 删除文件或文件夹
///
/// 删除用户在云空间内的文件或者文件夹。文件或者文件夹被删除后，会进入用户回收站里。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/delete

use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use crate::common::api_endpoints::DriveApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 删除类型
    pub r#type: String,
}

impl DeleteFileRequest {
    pub fn new(config: Config, file_token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<DeleteFileResponse>> {
        let api_endpoint = DriveApi::DeleteFile(self.file_token);
        let mut request = ApiRequest::<DeleteFileResponse>::delete(&api_endpoint.to_url());
        
        request = request.query("type", self.r#type);

        Transport::request(request, &self.config, None).await
    }
}

/// 删除文件或文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// 任务ID
    pub task_id: Option<String>,
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_file_request() {
        let config = Config::default();
        let request = DeleteFileRequest::new(config, "token_123", "file");
        assert_eq!(request.file_token, "token_123");
        assert_eq!(request.r#type, "file");
    }
}