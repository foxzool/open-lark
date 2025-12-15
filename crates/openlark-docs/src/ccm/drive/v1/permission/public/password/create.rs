/// 启用云文档密码
///
/// 为云文档启用密码保护功能。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 启用云文档密码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublicPasswordRequest {
    /// 文件token
    pub token: String,
    /// 密码
    pub password: String,
    /// 密码提示
    pub password_hint: Option<String>,
}

impl CreatePublicPasswordRequest {
    /// 创建启用云文档密码请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `password` - 密码
    pub fn new(
        token: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            password: password.into(),
            password_hint: None,
        }
    }

    /// 设置密码提示
    pub fn password_hint(mut self, password_hint: impl Into<String>) -> Self {
        self.password_hint = Some(password_hint.into());
        self
    }
}

/// 启用云文档密码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublicPasswordResponse {
    /// 密码保护信息
    pub password_protection: PasswordProtectionInfo,
}

/// 密码保护信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordProtectionInfo {
    /// 文件token
    pub file_token: String,
    /// 是否有密码保护
    pub has_password: bool,
    /// 密码提示
    pub password_hint: Option<String>,
    /// 创建时间
    pub created_at: String,
}

impl ApiResponseTrait for CreatePublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 启用云文档密码
///
/// 为云文档启用密码保护功能。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/create
pub async fn create_public_password(
    request: CreatePublicPasswordRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreatePublicPasswordResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "password": request.password
    });

    if let Some(password_hint) = &request.password_hint {
        body["passwordHint"] = serde_json::json!(password_hint);
    }

    // 创建API请求
    let mut api_request: ApiRequest<CreatePublicPasswordResponse> =
        ApiRequest::post(&format!("/open-apis/drive/v1/permissions/{}/public/password", request.token))
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
    fn test_create_public_password_request_builder() {
        let request = CreatePublicPasswordRequest::new("file_token", "password123")
            .password_hint("密码提示");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.password, "password123");
        assert_eq!(request.password_hint, Some("密码提示".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreatePublicPasswordResponse::data_format(), ResponseFormat::Data);
    }
}