/// 判断用户云文档权限
///
/// 该接口用于根据 filetoken 判断当前登录用户是否具有某权限。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 判断用户云文档权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPermissionMemberRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 权限类型，可选值：view（可见）、edit（可编辑）
    pub r#type: String,
    /// 用户类型，可选值：user（用户）、openid（openid）、unionid（unionid）
    pub user_id_type: Option<String>,
}

impl AuthPermissionMemberRequest {
    /// 创建判断用户云文档权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `type` - 权限类型
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<AuthPermissionMemberResponse>> {
        let api_endpoint = format!(
            "/open-apis/drive/v1/permissions/{}/members/auth",
            self.token
        );

        let mut api_request = ApiRequest::<AuthPermissionMemberResponse>::get(&api_endpoint)
            .query("type", &self.r#type);

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        Transport::request(api_request, &self.config, None).await
    }
}

/// 判断用户云文档权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPermissionMemberResponse {
    /// 是否有权限
    pub has_permission: bool,
    /// 权限类型
    pub r#type: String,
}

impl ApiResponseTrait for AuthPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_permission_member_request_builder() {
        let config = Config::default();
        let request =
            AuthPermissionMemberRequest::new(config, "file_token", "view").user_id_type("openid");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "view");
        assert_eq!(request.user_id_type, Some("openid".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            AuthPermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
