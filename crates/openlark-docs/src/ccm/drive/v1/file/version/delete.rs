use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 删除文件版本请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteFileVersionRequest {
    /// 文件token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
}

/// 删除文件版本响应
#[derive(Debug, Deserialize, Default)]
pub struct DeleteFileVersionResponse {
    /// 是否成功
    pub success: bool,
    /// 操作结果
    pub result: String,
}

/// 删除文件版本
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
pub async fn delete_file_version(
    request: DeleteFileVersionRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeleteFileVersionResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/versions/{}",
        config.base_url, request.file_token, request.version_id
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::DELETE,
        headers: vec![],
        query_params: vec![],
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_delete_file_version() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = DeleteFileVersionRequest {
            file_token: "test_file_token".to_string(),
            version_id: "test_version_id".to_string(),
        };

        let result = delete_file_version(request, &config, None).await;
        assert!(result.is_ok());
    }
}