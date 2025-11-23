//! 核心认证模块
//!
//! 从 openlark-core/auth 迁移的核心认证功能，提供令牌管理、缓存、验证和刷新等基础功能。

pub mod cache;
pub mod refresh;
pub mod token;
pub mod types;
pub mod validator;

// 重新导出主要类型
pub use cache::{CacheConfig, CacheStats, MemoryTokenCache, TokenCache, TokenStorage};
pub use refresh::{RefreshTokenResponse, TokenRefresher, TokenRefresherBuilder};
pub use token::{
    AccessToken, GetTokenRequest, GetTokenResponse, RefreshToken, TokenInfo, TokenRefreshConfig,
    TokenType, TokenValidationResult,
};
pub use types::{
    AuthContext, AuthValidationDetails, AuthValidationRequest, CacheStrategy, OAuthConfig,
    OAuthGrantType, OAuthRequest, OAuthResponse, PermissionScope, PreAuthCodeResponse,
    RefreshStrategy, TenantInfo, TokenExchangeRequest, TokenExchangeResponse, TokenSecurityConfig,
    TokenStorageLocation, UserInfo,
};
pub use validator::TokenValidator;
