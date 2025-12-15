/// 创建分享链接
///
/// 为文件创建分享链接，支持设置密码、有效期等安全选项。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/:file_token/share_links
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建分享链接请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareLinkRequest {
    /// 文件token
    pub file_token: String,
    /// 分享设置
    pub settings: ShareSettings,
}

impl CreateShareLinkRequest {
    /// 创建创建分享链接请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `settings` - 分享设置
    pub fn new(
        file_token: impl Into<String>,
        settings: ShareSettings,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            settings,
        }
    }
}

/// 分享设置
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ShareSettings {
    /// 创建分享设置
    pub fn new() -> Self {
        Self {
            expires_in: None,
            password_required: None,
            password: None,
            download_limit: None,
            preview_limit: None,
        }
    }

    /// 设置链接有效期（秒）
    pub fn expires_in(mut self, expires_in: u32) -> Self {
        self.expires_in = Some(expires_in);
        self
    }

    /// 设置是否需要密码
    pub fn password_required(mut self, required: bool) -> Self {
        self.password_required = Some(required);
        self
    }

    /// 设置密码
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// 设置下载限制
    pub fn download_limit(mut self, limit: u32) -> Self {
        self.download_limit = Some(limit);
        self
    }

    /// 设置预览限制
    pub fn preview_limit(mut self, limit: u32) -> Self {
        self.preview_limit = Some(limit);
        self
    }
}

/// 创建分享链接响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareLinkResponse {
    /// 分享链接信息
    pub data: Option<ShareLinkInfo>,
}

impl ApiResponseTrait for CreateShareLinkResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分享链接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
///
/// 为文件创建分享链接，支持设置密码、有效期等安全选项。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/:file_token/share_links
pub async fn create_share_link(
    request: CreateShareLinkRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateShareLinkResponse>> {
    // 构建请求体
    let body = json!({
        "settings": request.settings
    });

    // 创建API请求
    let url = DriveApi::CreateMediaShareLink(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<CreateShareLinkResponse> =
        ApiRequest::post(&url)
            .body(body);

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
    fn test_create_share_link_request_builder() {
        let settings = ShareSettings::new()
            .expires_in(86400)
            .password_required(true)
            .password("test_password")
            .download_limit(10);

        let request = CreateShareLinkRequest::new("file_token", settings);
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.settings.expires_in, Some(86400));
        assert_eq!(request.settings.password, Some("test_password".to_string()));
    }

    #[test]
    fn test_share_settings_builder() {
        let settings = ShareSettings::new()
            .expires_in(3600)
            .password_required(false)
            .download_limit(5)
            .preview_limit(20);

        assert_eq!(settings.expires_in, Some(3600));
        assert_eq!(settings.password_required, Some(false));
        assert_eq!(settings.download_limit, Some(5));
        assert_eq!(settings.preview_limit, Some(20));
    }

    #[test]
    fn test_share_link_info_structure() {
        let link_info = ShareLinkInfo {
            link_id: "link_123".to_string(),
            url: "https://example.com/share/link123".to_string(),
            share_token: "token_456".to_string(),
            expires_at: Some("2023-01-02T00:00:00Z".to_string()),
            password: Some("password123".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(link_info.link_id, "link_123");
        assert_eq!(link_info.share_token, "token_456");
        assert_eq!(link_info.password, Some("password123".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateShareLinkResponse::data_format(), ResponseFormat::Data);
    }
}