//! 自定义分组 API v2（strict 路径）

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod tasks;
pub mod patch;

pub use get::GetSectionRequest;
pub use list::ListSectionsRequest;
pub use tasks::GetSectionTasksRequest;

pub use create::CreateSectionRequest;
pub use delete::DeleteSectionRequest;
pub use patch::UpdateSectionRequest;
