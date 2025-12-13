//! Auth API实现模块 (meta.project=auth)
//!
//! 包含企业认证相关的API实现：
//! - app_access_token: 商店应用获取app_access_token
//! - app_access_token_internal: 自建应用获取app_access_token
//! - tenant_access_token: 商店应用获取tenant_access_token
//! - tenant_access_token_internal: 自建应用获取tenant_access_token
//! - app_ticket: 重新获取app_ticket

// 重新导出具体API实现
pub use self::v3::{auth::*, AuthServiceV3};

pub mod v3;
