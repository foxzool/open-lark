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
pub use auth::v3::auth::{AuthServiceV3, AppAccessTokenBuilder, AppAccessTokenInternalBuilder,
                        TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder, AppTicketResendBuilder};
pub use authen::v1::{AuthenServiceV1, UserInfoBuilder, UserAccessTokenV1Builder,
                    RefreshUserAccessTokenV1Builder, OidcService, OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder};
pub use oauth::old::{OAuthServiceOld, AuthorizationBuilder};