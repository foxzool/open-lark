/// Wiki API 模块
///
/// 提供飞书 Wiki 相关的 API。

pub mod v1;
pub mod v2;

// 重新导出所有 API 函数
pub use v1::*;
pub use v2::*;
