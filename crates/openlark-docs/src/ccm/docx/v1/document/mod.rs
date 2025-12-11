//! document模块 - 文档基础操作API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod block;
pub mod convert;
pub mod create;
pub mod get;
pub mod raw_content;

// 导出各模块类型，重命名冲突类型
pub use block::*;
pub use convert::*;
pub use create::{
    CreateDocumentParams, CreateDocumentRequest, CreateDocumentResponse,
    DocumentData as CreateDocumentData, UserInfo as CreateUserInfo,
};
pub use get::{
    DocumentContent, DocumentData as GetDocumentData, FolderInfo, GetDocumentParams,
    GetDocumentRequest, GetDocumentResponse, UserInfo as GetUserInfo,
};
pub use raw_content::*;
