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
    #[serde(skip)]
    config: Config,
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
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `password` - 密码
    pub fn new(
        config: Config,
        token: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            config,
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

    pub async fn execute(self) -> SDKResult<Response<CreatePublicPasswordResponse>> {
        let api_endpoint = format!("/open-apis/drive/v1/permissions/{}/public/password", self.token);

        let mut body = serde_json::json!({
            "password": self.password
        });

        if let Some(password_hint) = &self.password_hint {
            body["passwordHint"] = serde_json::json!(password_hint);
        }

        let api_request = ApiRequest::<CreatePublicPasswordResponse>::post(&api_endpoint)
            .body(body);
        
        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_public_password_request_builder() {
        let config = Config::default();
        let request = CreatePublicPasswordRequest::new(config, "file_token", "password123")
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