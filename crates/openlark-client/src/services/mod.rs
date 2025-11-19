//! OpenLark Client 服务访问层
//!
//! 提供统一的服务访问接口，作为底层crates的薄包装层

#[cfg(feature = "communication")]
pub mod communication;

#[cfg(feature = "hr")]
pub mod hr;

#[cfg(feature = "docs")]
pub mod docs;

#[cfg(feature = "ai")]
pub mod ai;

/// 认证服务
///
/// 提供飞书平台身份验证相关的API接口，包括令牌管理、OAuth认证等功能
#[cfg(feature = "auth")]
pub mod auth;

// 重新导出所有服务类型
#[cfg(feature = "communication")]
pub use communication::CommunicationService;

#[cfg(feature = "hr")]
pub use hr::HRService;

#[cfg(feature = "docs")]
pub use docs::DocsService;

#[cfg(feature = "ai")]
pub use ai::AIService;

#[cfg(feature = "auth")]
pub use auth::AuthService;

/// 📦 服务访问层预导出
pub mod prelude {
    #[cfg(feature = "communication")]
    pub use super::CommunicationService;

    #[cfg(feature = "hr")]
    pub use super::HRService;

    #[cfg(feature = "docs")]
    pub use super::DocsService;

    #[cfg(feature = "ai")]
    pub use super::AIService;

    #[cfg(feature = "auth")]
    pub use super::AuthService;
}
