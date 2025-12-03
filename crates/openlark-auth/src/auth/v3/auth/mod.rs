//! auth资源模块 - 企业应用认证相关API
//!
//! 包含auth项目的所有认证API：
//! - tenant_access_token_internal: 自建应用获取租户访问令牌
//! - app_access_token_internal: 自建应用获取应用访问令牌
//! - tenant_access_token: 商店应用获取租户访问令牌
//! - app_access_token: 商店应用获取应用访问令牌
//! - app_ticket_resend: 重新推送应用票据
//!
//! ### API分布
//! - 商店应用 (Marketplace):
//!   - `app_access_token`: 商店应用获取应用访问令牌
//!   - `tenant_access_token`: 商店应用获取租户访问令牌
//! - 自建应用 (Self-Build):
//!   - `app_access_token_internal`: 自建应用获取应用访问令牌
//!   - `tenant_access_token_internal`: 自建应用获取租户访问令牌
//! - 通用功能:
//!   - `app_ticket_resend`: 重新推送应用票据

pub mod app_access_token;
pub mod app_access_token_internal;
pub mod app_ticket;
pub mod tenant_access_token;
pub mod tenant_access_token_internal;

// 重新导出所有服务
pub use app_access_token::*;
pub use app_access_token_internal::*;
pub use app_ticket::*;
pub use tenant_access_token::*;
pub use tenant_access_token_internal::*;