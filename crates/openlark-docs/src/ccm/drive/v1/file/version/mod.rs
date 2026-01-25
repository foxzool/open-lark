/// 文件版本管理模块
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;

// 重新导出所有API函数
// create 模块显式导出
pub use create::{
    CreateFileVersionRequest,
    CreateFileVersionResponse,
    DeleteFileVersionRequest,
    DeleteFileVersionResponse,
    FileVersionInfo,
    GetFileVersionRequest,
    GetFileVersionResponse,
    ListFileVersionsData,
    ListFileVersionsRequest,
    ListFileVersionsResponse,
    execute,
    execute_with_options,
    new,
    page_token,
    user_id_type,
};
// delete 模块显式导出
pub use delete::{
    CreateFileVersionRequest,
    CreateFileVersionResponse,
    DeleteFileVersionRequest,
    DeleteFileVersionResponse,
    FileVersionInfo,
    GetFileVersionRequest,
    GetFileVersionResponse,
    ListFileVersionsData,
    ListFileVersionsRequest,
    ListFileVersionsResponse,
    execute,
    execute_with_options,
    new,
    page_token,
    user_id_type,
};
// get 模块显式导出
pub use get::{
    CreateFileVersionRequest,
    CreateFileVersionResponse,
    DeleteFileVersionRequest,
    DeleteFileVersionResponse,
    FileVersionInfo,
    GetFileVersionRequest,
    GetFileVersionResponse,
    ListFileVersionsData,
    ListFileVersionsRequest,
    ListFileVersionsResponse,
    execute,
    execute_with_options,
    new,
    page_token,
    user_id_type,
};
// list 模块显式导出
pub use list::{
    CreateFileVersionRequest,
    CreateFileVersionResponse,
    DeleteFileVersionRequest,
    DeleteFileVersionResponse,
    FileVersionInfo,
    GetFileVersionRequest,
    GetFileVersionResponse,
    ListFileVersionsData,
    ListFileVersionsRequest,
    ListFileVersionsResponse,
    execute,
    execute_with_options,
    new,
    page_token,
    user_id_type,
};
// models 模块显式导出
pub use models::{
    CreateFileVersionRequest,
    CreateFileVersionResponse,
    DeleteFileVersionRequest,
    DeleteFileVersionResponse,
    FileVersionInfo,
    GetFileVersionRequest,
    GetFileVersionResponse,
    ListFileVersionsData,
    ListFileVersionsRequest,
    ListFileVersionsResponse,
    execute,
    execute_with_options,
    new,
    page_token,
    user_id_type,
};
