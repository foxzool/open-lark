/// 刷新云文档密码
///
/// 刷新云文档的密码保护设置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 刷新云文档密码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublicPasswordRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 新密码
    pub password: String,
    /// 密码提示
    pub password_hint: Option<String>,
}

impl UpdatePublicPasswordRequest {
    /// 创建刷新云文档密码请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `password` - 新密码
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

    pub async fn execute(self) -> SDKResult<Response<UpdatePublicPasswordResponse>> {
        let api_endpoint = format!("/open-apis/drive/v1/permissions/{}/public/password", self.token);

        let mut body = serde_json::json!({
            "password": self.password
        });

        if let Some(password_hint) = &self.password_hint {
            body["passwordHint"] = serde_json::json!(password_hint);
        }

        let api_request = ApiRequest::<UpdatePublicPasswordResponse>::put(&api_endpoint)
            .body(body);
        
        Transport::request(api_request, &self.config, None).await
    }
}

/// 刷新云文档密码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublicPasswordResponse {
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
    /// 更新时间
    pub updated_at: String,
}

impl ApiResponseTrait for UpdatePublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_public_password_request_builder() {
        let config = Config::default();
        let request = UpdatePublicPasswordRequest::new(config, "file_token", "new_password123")
            .password_hint("新的密码提示");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.password, "new_password123");
        assert_eq!(request.password_hint, Some("新的密码提示".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdatePublicPasswordResponse::data_format(), ResponseFormat::Data);
    }
}