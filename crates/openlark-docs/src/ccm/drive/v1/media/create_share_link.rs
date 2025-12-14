use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建分享链接请求
#[derive(Debug, Serialize, Default)]
pub struct CreateShareLinkRequest {
    /// 文件token
    pub file_token: String,
    /// 分享设置
    pub settings: ShareSettings,
}

/// 分享设置
#[derive(Debug, Serialize, Default)]
pub struct ShareSettings {
    /// 链接有效期（秒）
    pub expires_in: Option<u32>,
    /// 需要密码
    pub password_required: Option<bool>,
    /// 密码
    pub password: Option<String>,
    /// 下载限制
    pub download_limit: Option<u32>,
    /// 预览限制
    pub preview_limit: Option<u32>,
}

/// 创建分享链接响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateShareLinkResponse {
    /// 分享链接信息
    pub share_link: ShareLinkInfo,
    /// 操作结果
    pub result: String,
}

/// 分享链接信息
#[derive(Debug, Deserialize, Default)]
pub struct ShareLinkInfo {
    /// 链接ID
    pub link_id: String,
    /// 分享链接
    pub url: String,
    /// 链接token
    pub share_token: String,
    /// 过期时间
    pub expires_at: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 创建时间
    pub created_at: String,
}

/// 创建分享链接
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/:file_token/share_links
pub async fn create_share_link(
    request: CreateShareLinkRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateShareLinkResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/medias/{}/share_links",
        config.base_url, request.file_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
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
    async fn test_create_share_link() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateShareLinkRequest {
            file_token: "test_file_token".to_string(),
            settings: ShareSettings {
                expires_in: Some(86400),
                password_required: Some(true),
                password: Some("test_password".to_string()),
                download_limit: Some(10),
                preview_limit: None,
            },
        };

        let result = create_share_link(request, &config, None).await;
        assert!(result.is_ok());
    }
}