use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 删除密码保护请求
#[derive(Debug, Serialize, Default)]
pub struct DeletePasswordRequest {
    /// 文件token
    pub file_token: String,
    /// 密码（验证用）
    pub password: Option<String>,
    /// 是否删除所有密码（包括部门密码）
    pub delete_all: Option<bool>,
}

/// 删除密码保护响应
#[derive(Debug, Deserialize, Default)]
pub struct DeletePasswordResponse {
    /// 是否成功
    pub success: bool,
    /// 操作结果
    pub result: String,
}

/// 删除密码保护
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/password
pub async fn delete_password(
    request: DeletePasswordRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeletePasswordResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/password",
        config.base_url, request.file_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::DELETE,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_delete_password() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = DeletePasswordRequest {
            file_token: "test_file_token".to_string(),
            password: Some("test_password".to_string()),
            delete_all: Some(false),
        };

        let result = delete_password(request, &config, None).await;
        assert!(result.is_ok());
    }
}