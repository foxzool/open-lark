/// 删除文件版本
///
/// 删除指定文件的特定版本。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 删除文件版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileVersionRequest {
    /// 文件token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
}

impl DeleteFileVersionRequest {
    /// 创建删除文件版本请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `version_id` - 版本ID
    pub fn new(
        file_token: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            version_id: version_id.into(),
        }
    }
}

/// 删除文件版本响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileVersionResponse {
    /// 是否成功
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteFileVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文件版本
///
/// 删除指定文件的特定版本。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
pub async fn delete_file_version(
    request: DeleteFileVersionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeleteFileVersionResponse>> {
    // 创建API请求
    let url = DriveApi::DeleteFileVersion(request.file_token.clone(), request.version_id.clone()).to_url();
    let mut api_request: ApiRequest<DeleteFileVersionResponse> =
        ApiRequest::delete(&url);

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
    fn test_delete_file_version_request_builder() {
        let request = DeleteFileVersionRequest::new("file_token", "version_id");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.version_id, "version_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteFileVersionResponse::data_format(), ResponseFormat::Data);
    }
}