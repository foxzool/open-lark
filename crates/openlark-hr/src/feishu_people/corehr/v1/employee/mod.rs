//! CoreHR 员工 API
//!
//! 提供员工管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod batch_get;
pub mod create;
pub mod delete;
pub mod list;
pub mod models;
pub mod search;

// Re-export 公共类型
pub use batch_get::BatchGetRequest;
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use list::ListRequest;
pub use models::{
    BatchGetRequestBody, BatchGetResponse, CreateRequestBody, CreateResponse, DeleteRequestBody,
    DeleteResponse, Employee, EmployeeRoster, ListRequestBody, ListResponse, SearchRequestBody,
    SearchResponse,
};
pub use search::SearchRequest;
