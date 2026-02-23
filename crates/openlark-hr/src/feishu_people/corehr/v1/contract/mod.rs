//! CoreHR 合同 API
//!
//! 提供合同管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod search;

// Re-export 公共类型
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use get::GetRequest;
pub use list::ListRequest;
pub use models::{
    Contract, ContractFile, CreateRequestBody, CreateResponse, CustomField, DeleteRequestBody,
    DeleteResponse, GetRequestBody, GetResponse, ListRequestBody, ListResponse, PatchRequestBody,
    PatchResponse, SearchRequestBody, SearchResponse,
};
pub use patch::PatchRequest;
pub use search::SearchRequest;
