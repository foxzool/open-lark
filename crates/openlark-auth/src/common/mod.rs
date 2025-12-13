//! OpenLark Auth 公共模块

pub mod api_endpoints;

// 重新导出常用类型
pub use api_endpoints::{AuthApiV3, AuthenApiV1, OAuthApiOld};
