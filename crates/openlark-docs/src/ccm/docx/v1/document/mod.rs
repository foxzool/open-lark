//! document模块 - 文档基础操作API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod create;
pub mod get;
pub mod raw_content;
pub mod convert;
pub mod block;

// 导出各模块类型，重命名冲突类型
pub use create::{
    CreateDocumentRequest, CreateDocumentParams, CreateDocumentResponse,
    DocumentData as CreateDocumentData, UserInfo as CreateUserInfo
};
pub use get::{
    GetDocumentRequest, GetDocumentParams, GetDocumentResponse,
    DocumentData as GetDocumentData, UserInfo as GetUserInfo,
    FolderInfo, DocumentContent
};
pub use raw_content::*;
pub use convert::*;
pub use block::*;