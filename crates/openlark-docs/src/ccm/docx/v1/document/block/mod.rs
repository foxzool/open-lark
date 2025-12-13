/// block模块 - 文档块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织

pub mod batch_update;
pub mod children;
pub mod descendant;
pub mod get;
pub mod list;
pub mod patch;

// 导出公共类型，避免命名冲突
pub use batch_update::*;
pub use get::*;
pub use list::*;
pub use patch::*;

// children模块导出，重命名冲突类型
pub use children::batch_delete::*;
pub use children::create::{
    BlockLocation as ChildrenBlockLocation, CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest, CreateDocumentBlockChildrenResponse,
    CreateResult as ChildrenCreateResult, CreatedBlock as ChildrenCreatedBlock,
    NewBlock as ChildrenNewBlock,
};
pub use children::get::*;

// descendant模块导出，重命名冲突类型
pub use descendant::create::{
    BlockLocation as DescendantBlockLocation, CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantResponse,
    CreateResult as DescendantCreateResult, CreatedNestedBlock, NewNestedBlock,
};
