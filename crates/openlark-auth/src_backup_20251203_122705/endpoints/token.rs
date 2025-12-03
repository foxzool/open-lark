//! 令牌API端点

/// 令牌相关API端点
pub struct TokenEndpoints;

impl TokenEndpoints {
    /// 获取应用访问令牌
    pub const APP_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/app_access_token/internal";

    /// 获取租户访问令牌
    pub const TENANT_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/tenant_access_token/internal";

    /// 刷新访问令牌
    pub const REFRESH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/refresh_access_token";

    /// 获取用户访问令牌
    pub const USER_ACCESS_TOKEN: &'static str = "/open-apis/authen/v1/access_token";

    /// 验证令牌
    pub const VERIFY_TOKEN: &'static str = "/open-apis/auth/v3/verify";

    /// 撤销令牌
    pub const REVOKE_TOKEN: &'static str = "/open-apis/auth/v3/revoke";

    /// 令牌交换
    pub const TOKEN_EXCHANGE: &'static str = "/open-apis/auth/v3/token/exchange";

    /// 获取令牌信息
    pub const TOKEN_INFO: &'static str = "/open-apis/auth/v3/token_info";

    /// 批量获取令牌信息
    pub const BATCH_TOKEN_INFO: &'static str = "/open-apis/auth/v3/batch_token_info";
}

impl Default for TokenEndpoints {
    fn default() -> Self {
        Self
    }
}

/// 令牌端点构建器
pub struct TokenEndpointBuilder {
    base_url: String,
}

impl TokenEndpointBuilder {
    /// 创建新的令牌端点构建器
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// 构建完整的端点URL
    pub fn build_endpoint(&self, endpoint: &'static str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

    /// 构建应用访问令牌端点
    pub fn app_access_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::APP_ACCESS_TOKEN)
    }

    /// 构建租户访问令牌端点
    pub fn tenant_access_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::TENANT_ACCESS_TOKEN)
    }

    /// 构建刷新访问令牌端点
    pub fn refresh_access_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::REFRESH_ACCESS_TOKEN)
    }

    /// 构建用户访问令牌端点
    pub fn user_access_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::USER_ACCESS_TOKEN)
    }

    /// 构建验证令牌端点
    pub fn verify_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::VERIFY_TOKEN)
    }

    /// 构建撤销令牌端点
    pub fn revoke_token(&self) -> String {
        self.build_endpoint(TokenEndpoints::REVOKE_TOKEN)
    }

    /// 构建令牌交换端点
    pub fn token_exchange(&self) -> String {
        self.build_endpoint(TokenEndpoints::TOKEN_EXCHANGE)
    }

    /// 构建获取令牌信息端点
    pub fn token_info(&self) -> String {
        self.build_endpoint(TokenEndpoints::TOKEN_INFO)
    }

    /// 构建批量获取令牌信息端点
    pub fn batch_token_info(&self) -> String {
        self.build_endpoint(TokenEndpoints::BATCH_TOKEN_INFO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_endpoints() {
        assert_eq!(
            TokenEndpoints::APP_ACCESS_TOKEN,
            "/open-apis/auth/v3/app_access_token/internal"
        );
        assert_eq!(
            TokenEndpoints::TENANT_ACCESS_TOKEN,
            "/open-apis/auth/v3/tenant_access_token/internal"
        );
        assert_eq!(
            TokenEndpoints::USER_ACCESS_TOKEN,
            "/open-apis/authen/v1/access_token"
        );
        assert_eq!(
            TokenEndpoints::REFRESH_ACCESS_TOKEN,
            "/open-apis/auth/v3/refresh_access_token"
        );
    }

    #[test]
    fn test_token_endpoint_builder() {
        let builder = TokenEndpointBuilder::new("https://open.feishu.cn");

        assert_eq!(
            builder.app_access_token(),
            "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal"
        );
        assert_eq!(
            builder.tenant_access_token(),
            "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal"
        );
        assert_eq!(
            builder.user_access_token(),
            "https://open.feishu.cn/open-apis/authen/v1/access_token"
        );
    }

    #[test]
    fn test_token_endpoint_builder_custom_base() {
        let builder = TokenEndpointBuilder::new("https://custom.domain.com");

        assert_eq!(
            builder.verify_token(),
            "https://custom.domain.com/open-apis/auth/v3/verify"
        );
        assert_eq!(
            builder.revoke_token(),
            "https://custom.domain.com/open-apis/auth/v3/revoke"
        );
    }
}
