//! block模块 - 文档块操作API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod list;
pub mod get;
pub mod patch;
pub mod batch_update;
pub mod children;
pub mod descendant;

// 导出公共类型，避免命名冲突
pub use list::*;
pub use get::*;
pub use patch::*;
pub use batch_update::*;

// children模块导出，重命名冲突类型
pub use children::get::*;
pub use children::batch_delete::*;
pub use children::create::{
    CreateDocumentBlockChildrenRequest, CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenResponse,
    CreateResult as ChildrenCreateResult, CreatedBlock as ChildrenCreatedBlock,
    NewBlock as ChildrenNewBlock, BlockLocation as ChildrenBlockLocation
};

// descendant模块导出，重命名冲突类型
pub use descendant::create::{
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantParams, CreateDocumentBlockDescendantResponse,
    CreateResult as DescendantCreateResult, CreatedNestedBlock, NewNestedBlock,
    BlockLocation as DescendantBlockLocation
};