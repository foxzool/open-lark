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
    /// 文件token
    pub token: String,
}

impl GetPublicPermissionRequest {
    /// 创建获取公开权限设置请求
    ///
    /// # 参数
    /// * `token` - 文件token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
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

/// 获取云文档权限设置
///
/// 获取文件或文件夹的公开权限设置
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/get
pub async fn get_public_permission(
    request: GetPublicPermissionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetPublicPermissionResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::GetPublicPermission(request.token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<GetPublicPermissionResponse> =
        ApiRequest::get(&api_endpoint.to_url());

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
    fn test_get_public_permission_request_builder() {
        let request = GetPublicPermissionRequest::new("file_token");

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