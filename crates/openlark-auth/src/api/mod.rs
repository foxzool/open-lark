//! API实现层
//!
//! 本模块包含所有API的具体实现，按照版本和业务领域组织。

/// Auth相关API (meta.project=auth)
pub mod auth;

/// Authen相关API (meta.project=authen)
pub mod authen;

/// OAuth相关API (meta.project=oauth)
pub mod oauth;