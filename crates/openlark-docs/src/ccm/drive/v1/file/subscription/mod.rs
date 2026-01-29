/// 文件订阅管理模块
///
/// 提供云文档订阅管理功能。
pub mod create;
pub mod get;
pub mod models;
pub mod patch;

// 显式导出 - 避免使用 glob reexport
pub use create::{CreateFileSubscriptionRequest};

pub use get::{GetSubscriptionRequest, GetSubscriptionResponse};

pub use models::{Subscription};

pub use patch::{PatchSubscriptionRequest};
