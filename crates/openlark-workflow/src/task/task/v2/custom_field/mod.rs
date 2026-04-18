//! 自定义字段 API v2（strict 路径）

pub mod add;
pub mod create;
pub mod get;
pub mod list;
pub mod option;
pub mod patch;
pub mod remove;

pub use add::AddCustomFieldRequest;
pub use create::CreateCustomFieldRequest;
pub use get::GetCustomFieldRequest;
pub use list::ListCustomFieldsRequest;
pub use option::{CreateCustomFieldOptionRequest, UpdateCustomFieldOptionRequest};
pub use patch::PatchCustomFieldRequest;
pub use remove::RemoveCustomFieldRequest;
