use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取云文档权限设置
///
/// 获取文件或文件夹的公开权限设置
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/get
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取公开权限设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
}

impl GetPublicPermissionRequest {
    /// 创建获取公开权限设置请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    pub fn new(config: Config, token: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<GetPublicPermissionResponse>> {
        let api_endpoint = DriveApi::GetPublicPermission(self.token.clone());

        let api_request = ApiRequest::<GetPublicPermissionResponse>::get(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, None).await
    }
}

/// 权限设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSetting {
    /// 是否公开
    pub public: bool,
    /// 权限类型
    pub r#type: String,
    /// 链接
    pub share_url: Option<String>,
    /// 密码保护
    pub password_protected: bool,
    /// 密码信息
    pub password_info: Option<PasswordInfo>,
}

/// 密码信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordInfo {
    /// 是否启用
    pub enabled: bool,
    /// 密码
    pub password: Option<String>,
}

/// 获取公开权限设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionResponse {
    /// 权限设置
    pub data: Option<PermissionSetting>,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_permission_request_builder() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "file_token");

        assert_eq!(request.token, "file_token");
    }

    #[test]
    fn test_permission_setting_structure() {
        let password_info = PasswordInfo {
            enabled: true,
            password: Some("123456".to_string()),
        };

        let permission = PermissionSetting {
            public: true,
            r#type: "anyone_with_link".to_string(),
            share_url: Some("https://example.com/share".to_string()),
            password_protected: true,
            password_info: Some(password_info),
        };

        assert!(permission.public);
        assert_eq!(permission.r#type, "anyone_with_link");
        assert!(permission.password_protected);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetPublicPermissionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}