//! 认证服务模块
//!
//! 提供完整的飞书开放平台认证功能，支持多种认证流程：
//! - 用户信息获取和管理
//! - 应用访问令牌（App Access Token）管理
//! - 租户访问令牌（Tenant Access Token）管理
//! - 应用票据（App Ticket）管理
//!
//! # 模块结构
//!
//! - `v1`: 认证服务V1版本，提供完整的认证API功能
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::authentication::v1::V1;
//!
//! let client = LarkClient::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build()?;
//!
//! // 获取用户信息
//! let user_info = client.authen.v1.user_info
//!     .get_user_info_builder()
//!     .user_access_token("user_access_token")
//!     .execute(&client.authen.v1.user_info)
//!     .await?;
//!
//! println!("用户姓名: {}", user_info.data.name);
//! ```

pub mod v1;

// 重新导出V1版本的主要类型
pub use v1::{
    AppAccessTokenResponse, AppAccessTokenService, AppTicketService,
    GetAppAccessTokenBuilder, GetAppAccessTokenInternalBuilder, GetAppAccessTokenInternalRequest,
    GetAppAccessTokenRequest, GetTenantAccessTokenBuilder, GetTenantAccessTokenInternalBuilder,
    GetTenantAccessTokenInternalRequest, GetTenantAccessTokenRequest, GetUserInfoBuilder,
    ResendAppTicketBuilder, ResendAppTicketRequest, ResendAppTicketResponse,
    TenantAccessTokenResponse, TenantAccessTokenService, UserInfo, UserInfoService, V1,
};

/// 认证服务（V1版本的类型别名）
///
/// 为了向后兼容性提供的类型别名
pub type AuthenticationService = V1;