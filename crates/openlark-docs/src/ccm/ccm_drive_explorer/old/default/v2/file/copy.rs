use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 复制文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileRequest {
    /// 文件 token
    #[serde(skip)]
    pub file_token: String,
    /// 目标文件夹 token
    pub dstFolderToken: String,
    /// 目标文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstName: Option<String>,
    /// 评论复制选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentNeeded: Option<bool>,
}

impl CopyFileRequest {
    /// 创建新的 CopyFileRequest
    pub fn new(file_token: impl Into<String>, dst_folder_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            dstFolderToken: dst_folder_token.into(),
            dstName: None,
            commentNeeded: None,
        }
    }

    /// 设置目标文件名
    pub fn dst_name(mut self, name: impl Into<String>) -> Self {
        self.dstName = Some(name.into());
        self
    }

    /// 设置评论复制选项
    pub fn comment_needed(mut self, needed: bool) -> Self {
        self.commentNeeded = Some(needed);
        self
    }
}

/// 复制文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// folderToken
    pub folderToken: String,
    /// revision
    pub revision: i64,
    /// token
    pub token: String,
    /// type
    pub r#type: String,
    /// url
    pub url: String,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制文档
///
/// 根据文件 token 复制 Doc 或 Sheet 到目标文件夹中。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet
pub async fn copy(
    request: CopyFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CopyFileResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::FileCopy(request.file_token.clone());
    let mut api_request: ApiRequest<CopyFileResponse> = ApiRequest::post(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_request() {
        let request = CopyFileRequest::new("file_token", "dst_folder_token");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.dstFolderToken, "dst_folder_token");
    }
}
