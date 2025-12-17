/// Minutes V1 妙记管理模块
///
/// 提供妙记基础信息、音视频、文字记录和统计数据管理功能。
pub mod get;
pub mod media;
pub mod models;
pub mod service;
pub mod statistics;
pub mod transcript;

// 导出所有子模块内容，避免命名冲突
pub use get::*;
// 注意：以下子模块本身只导出了get，不再直接 `pub use media::*` 否则可能会有命名冲突，
// 但这里我们希望用户使用 `MediaService::get` 或直接用 Request 对象
// 根据原有模式，我们保持导出，但要小心命名及其结构
pub use media::*;
pub use models::*;
pub use service::*;
pub use statistics::*;
pub use transcript::*;
