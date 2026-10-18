//! 鉴权模块（ADR-0002）：收口「给请求做鉴权」整条 concern——
//! 获取（acquisition）+ 决策/授权校验（policy）+ 恢复（app_ticket）。
//!
//! `acquisition` / `policy` / `app_ticket` 为 `pub(crate)`：仅 core 内部可达，不进公开 API。

pub(crate) mod acquisition;
pub(crate) mod app_ticket;
mod client_assertion;
pub(crate) mod policy;
pub mod token_provider;

// Re-export commonly used types
pub use client_assertion::{
    CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE, CLIENT_ASSERTION_CUSTOM_BASE_URL,
    CLIENT_ASSERTION_EMPTY, CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED,
    CLIENT_ASSERTION_NEEDS_AUTH_FEATURE, CLIENT_ASSERTION_OAUTH_BASE_INVALID,
    CLIENT_ASSERTION_OAUTH_FAILED_PREFIX, CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX,
    ClientAssertionProvider,
};
pub use token_provider::{NoOpTokenProvider, TokenProvider, TokenRequest};

// AuthHandler（token 获取 adapter）由 acquisition 提供；供 request_execution 编排时调用。
pub(crate) use acquisition::AuthHandler;
