/// Wiki V2 空间设置模块
///
/// 提供知识空间设置更新功能。

pub mod update;

// 显式导出 - 避免使用 glob reexport
pub use update::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
