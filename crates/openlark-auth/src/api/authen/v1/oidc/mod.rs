//! OIDC认证相关API模块
//!
//! 本模块包含OIDC（OpenID Connect）相关的认证API实现

pub mod access_token;
pub mod refresh_access_token;

// 重新导出构建器，方便外部使用
pub use access_token::OidcAccessTokenBuilder;
pub use refresh_access_token::OidcRefreshAccessTokenBuilder;

/// OIDC认证服务
#[derive(Debug)]
pub struct OidcService {
    config: openlark_core::config::Config,
}

impl OidcService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 获取OIDC用户访问令牌
    pub fn access_token(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder::new(self.config.clone())
    }

    /// 刷新OIDC用户访问令牌
    pub fn refresh_access_token(&self) -> OidcRefreshAccessTokenBuilder {
        OidcRefreshAccessTokenBuilder::new(self.config.clone())
    }
}
