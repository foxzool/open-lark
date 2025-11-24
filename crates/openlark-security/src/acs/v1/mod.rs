//! 访问控制 V1 API 模块
//!
//! 提供飞书开放平台访问控制相关的V1版本API实现。

pub mod permission;
pub mod policy;
pub mod role;

// 重新导出所有v1 API
pub use permission::*;
pub use policy::*;
pub use role::*;
