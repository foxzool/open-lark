/// Wiki V2 API 模块
///
/// 提供飞书 Wiki V2 相关的 API。

pub mod space;
pub mod task;

// 重新导出所有 API 函数
pub use space::*;
pub use task::*;
