//! 用户身份认证模块
//!
//! 提供用户身份认证相关 API 的版本化入口，聚合用户信息、访问令牌与 OIDC 认证能力。
//!
//! ## 主要功能
//! - `v1`: 用户认证 v1 版本接口入口
//! - 用户访问令牌申请与刷新
//! - OIDC 访问令牌相关接口

// v1 模块显式导出
pub use v1::{
    AuthenServiceV1, OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, OidcService,
    RefreshUserAccessTokenV1Builder, UserAccessTokenV1Builder, UserInfoBuilder, UserInfoService,
};

pub mod v1;
