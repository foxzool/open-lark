//! OAuth API实现模块 (meta.project=oauth)
//!
//! 包含OAuth授权相关的API实现：
//! - authorization.v1/index: 获取登录预授权码

// 重新导出具体API实现
pub use self::old::*;

pub mod old;