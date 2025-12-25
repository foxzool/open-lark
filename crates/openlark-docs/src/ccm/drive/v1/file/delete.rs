use crate::common::{api_endpoints::DriveApi, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 删除文件或文件夹
///
/// 删除用户在云空间内的文件或者文件夹。文件或者文件夹被删除后，会进入用户回收站里。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/delete
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteFileRequest {
    #[serde(skip)]
    config: Config,
    /// 需要删除的文件或文件夹 token
    pub file_token: String,
    /// 被删除文件的类型
    #[serde(skip)]
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

    pub async fn execute(self) -> SDKResult<DeleteFileResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error("type", "type 不能为空"));
        }

        let api_endpoint = DriveApi::DeleteFile(self.file_token);
        let mut request = ApiRequest::<DeleteFileResponse>::delete(&api_endpoint.to_url());

        request = request.query("type", self.r#type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "删除文件或文件夹")
    }
}

/// 删除文件或文件夹响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// 异步任务 ID，删除文件夹时返回
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
