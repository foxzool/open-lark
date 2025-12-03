//! 认证API端点

/// 认证相关API端点
pub struct AuthEndpoints;

impl AuthEndpoints {
    /// 获取应用访问令牌
    pub const APP_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/app_access_token/internal";

    /// 获取租户访问令牌
    pub const TENANT_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/tenant_access_token/internal";

    /// 刷新访问令牌
    pub const REFRESH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/refresh_access_token";

    /// 获取用户访问令牌
    pub const USER_ACCESS_TOKEN: &'static str = "/open-apis/authen/v1/access_token";

    /// 获取预授权码
    pub const PRE_AUTH_CODE: &'static str = "/open-apis/authen/v1/pre_auth_codes";

    /// 获取应用权限
    pub const APP_PERMISSIONS: &'static str = "/open-apis/auth/v3/app/permissions/query";

    /// 获取用户信息
    pub const USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    /// 获取租户信息
    pub const TENANT_INFO: &'static str = "/open-apis/tenant/v2/tenant/query";

    /// 验证令牌
    pub const VERIFY_TOKEN: &'static str = "/open-apis/auth/v3/verify";

    /// 撤销令牌
    pub const REVOKE_TOKEN: &'static str = "/open-apis/auth/v3/revoke";

    // === 商店应用API端点 ===
    /// 商店应用获取应用访问令牌
    pub const APP_ACCESS_TOKEN_STORE: &'static str = "/open-apis/auth/v3/app_access_token";

    /// 商店应用获取租户访问令牌
    pub const TENANT_ACCESS_TOKEN_STORE: &'static str = "/open-apis/auth/v3/tenant_access_token";

    /// 重新推送应用票据
    pub const APP_TICKET_RESEND: &'static str = "/open-apis/auth/v3/app_ticket/resend";

    // === OIDC相关端点 ===
    /// 获取OIDC访问令牌
    pub const OIDC_ACCESS_TOKEN: &'static str = "/open-apis/authen/v1/oidc/access_token";

    /// 刷新OIDC访问令牌
    pub const OIDC_REFRESH_ACCESS_TOKEN: &'static str =
        "/open-apis/authen/v1/oidc/refresh_access_token";

    // === OAuth相关端点 ===
    /// 获取登录预授权码
    pub const OAUTH_INDEX: &'static str = "/open-apis/authen/v1/index";
}

impl Default for AuthEndpoints {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_endpoints() {
        // 自建应用端点
        assert_eq!(
            AuthEndpoints::APP_ACCESS_TOKEN,
            "/open-apis/auth/v3/app_access_token/internal"
        );
        assert_eq!(
            AuthEndpoints::TENANT_ACCESS_TOKEN,
            "/open-apis/auth/v3/tenant_access_token/internal"
        );

        // 商店应用端点
        assert_eq!(
            AuthEndpoints::APP_ACCESS_TOKEN_STORE,
            "/open-apis/auth/v3/app_access_token"
        );
        assert_eq!(
            AuthEndpoints::TENANT_ACCESS_TOKEN_STORE,
            "/open-apis/auth/v3/tenant_access_token"
        );
        assert_eq!(
            AuthEndpoints::APP_TICKET_RESEND,
            "/open-apis/auth/v3/app_ticket/resend"
        );

        // 用户认证端点
        assert_eq!(
            AuthEndpoints::USER_ACCESS_TOKEN,
            "/open-apis/authen/v1/access_token"
        );
        assert_eq!(AuthEndpoints::USER_INFO, "/open-apis/authen/v1/user_info");

        // OIDC端点
        assert_eq!(
            AuthEndpoints::OIDC_ACCESS_TOKEN,
            "/open-apis/authen/v1/oidc/access_token"
        );
        assert_eq!(
            AuthEndpoints::OIDC_REFRESH_ACCESS_TOKEN,
            "/open-apis/authen/v1/oidc/refresh_access_token"
        );

        // OAuth端点
        assert_eq!(AuthEndpoints::OAUTH_INDEX, "/open-apis/authen/v1/index");
    }
}
