//! CoreHR 职级（job_level）相关 API
//!
//! 包含创建、删除、查询、更新职级等功能

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;

// Re-export 公共类型
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use get::GetRequest;
pub use list::ListRequest;
pub use models::{
    CreateRequestBody, CreateResponse, DeleteResponse, GetResponse, JobLevel, ListRequestBody,
    ListResponse, PatchRequestBody, PatchResponse,
};
pub use patch::PatchRequest;
