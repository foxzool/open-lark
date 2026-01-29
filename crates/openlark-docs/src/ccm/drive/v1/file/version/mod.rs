/// 文件版本管理模块
///
/// 提供云文档版本管理功能，包括创建、查询、删除版本等。
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;

// 显式导出 - 避免使用 glob reexport
pub use create::{CreateFileVersionRequest, CreateFileVersionResponse};

pub use delete::{DeleteFileVersionRequest, DeleteFileVersionResponse};

pub use get::{GetFileVersionRequest, GetFileVersionResponse};

pub use list::{ListFileVersionsRequest, ListFileVersionsResponse};

pub use models::FileVersionInfo;
