//! CoreHR 职务相关 API
//!
//! 包含创建、删除、查询、更新职务等 API

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
    CreateRequestBody, CreateResponse, CustomField, DeleteRequestBody, DeleteResponse,
    GetRequestBody, GetResponse, Job, ListRequestBody, ListResponse, PatchRequestBody,
    PatchResponse,
};
pub use patch::PatchRequest;
