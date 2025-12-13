//! API实现层
//!
//! 本模块包含所有API的具体实现，按照版本和业务领域组织。

/// Auth相关API (meta.project=auth)
pub mod auth;

/// Authen相关API (meta.project=authen)
pub mod authen;

/// OAuth相关API (meta.project=oauth)
pub mod oauth;

// 重新导出主要服务类，方便用户使用
pub use auth::v3::auth::{
    AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder, AuthServiceV3,
    TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder,
};
pub use authen::v1::{
    AuthenServiceV1, OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, OidcService,
    RefreshUserAccessTokenV1Builder, UserAccessTokenV1Builder, UserInfoBuilder,
};
pub use oauth::old::{AuthorizationBuilder, OAuthServiceOld};
