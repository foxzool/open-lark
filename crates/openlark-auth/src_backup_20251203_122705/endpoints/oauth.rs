//! OAuth API端点

/// OAuth相关API端点
pub struct OAuthEndpoints;

impl OAuthEndpoints {
    /// 获取授权URL
    pub const AUTHORIZE: &'static str = "/open-apis/authen/v1/authorize";

    /// 获取访问令牌
    pub const ACCESS_TOKEN: &'static str = "/open-apis/authen/v1/access_token";

    /// 刷新访问令牌
    pub const REFRESH_TOKEN: &'static str = "/open-apis/authen/v1/refresh_access_token";

    /// 撤销访问令牌
    pub const REVOKE_TOKEN: &'static str = "/open-apis/authen/v1/revoke_tokens";

    /// 获取用户信息
    pub const USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    /// 获取应用信息
    pub const APP_INFO: &'static str = "/open-apis/application/v6/applications";

    /// 获取预授权码
    pub const PRE_AUTH_CODE: &'static str = "/open-apis/authen/v1/pre_auth_codes";

    /// 获取应用权限
    pub const APP_PERMISSIONS: &'static str = "/open-apis/auth/v3/app/permissions/query";

    /// 验证令牌
    pub const VERIFY_TOKEN: &'static str = "/open-apis/auth/v3/verify";

    /// 令牌交换
    pub const TOKEN_EXCHANGE: &'static str = "/open-apis/auth/v3/token/exchange";
}

impl Default for OAuthEndpoints {
    fn default() -> Self {
        Self
    }
}

/// OAuth URL构建器
pub struct OAuthUrlBuilder {
    base_url: String,
    app_id: String,
    redirect_uri: String,
    scope: String,
    state: Option<String>,
    response_type: String,
}

impl OAuthUrlBuilder {
    /// 创建新的OAuth URL构建器
    pub fn new<S: Into<String>>(base_url: S, app_id: S, redirect_uri: S) -> Self {
        Self {
            base_url: base_url.into(),
            app_id: app_id.into(),
            redirect_uri: redirect_uri.into(),
            scope: String::new(),
            state: None,
            response_type: "code".to_string(),
        }
    }

    /// 设置权限范围
    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.scope = scope.into();
        self
    }

    /// 设置状态参数
    pub fn state<S: Into<String>>(mut self, state: S) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 设置响应类型
    pub fn response_type<S: Into<String>>(mut self, response_type: S) -> Self {
        self.response_type = response_type.into();
        self
    }

    /// 构建授权URL
    pub fn build(self) -> String {
        let mut url = format!(
            "{}{}?app_id={}&redirect_uri={}&response_type={}",
            self.base_url,
            OAuthEndpoints::AUTHORIZE,
            urlencoding::encode(&self.app_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&self.response_type)
        );

        if !self.scope.is_empty() {
            url.push_str(&format!("&scope={}", urlencoding::encode(&self.scope)));
        }

        if let Some(state) = &self.state {
            url.push_str(&format!("&state={}", urlencoding::encode(state)));
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_endpoints() {
        assert_eq!(OAuthEndpoints::AUTHORIZE, "/open-apis/authen/v1/authorize");
        assert_eq!(
            OAuthEndpoints::ACCESS_TOKEN,
            "/open-apis/authen/v1/access_token"
        );
        assert_eq!(
            OAuthEndpoints::REFRESH_TOKEN,
            "/open-apis/authen/v1/refresh_access_token"
        );
    }

    #[test]
    fn test_oauth_url_builder() {
        let url = OAuthUrlBuilder::new(
            "https://open.feishu.cn",
            "app_123",
            "https://example.com/callback",
        )
        .scope("contact:base")
        .state("random_state")
        .build();

        assert!(url.contains("app_id=app_123"));
        assert!(url.contains("redirect_uri=https%3A//example.com/callback"));
        assert!(url.contains("scope=contact%3Abase"));
        assert!(url.contains("state=random_state"));
    }

    #[test]
    fn test_oauth_url_builder_minimal() {
        let url = OAuthUrlBuilder::new(
            "https://open.feishu.cn",
            "app_123",
            "https://example.com/callback",
        )
        .build();

        assert!(url.contains("app_id=app_123"));
        assert!(url.contains("redirect_uri=https%3A//example.com/callback"));
        assert!(url.contains("response_type=code"));
        assert!(!url.contains("scope="));
        assert!(!url.contains("state="));
    }
}
