/// block模块 - 文档块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_update;
pub mod children;
pub mod descendant;
pub mod get;
pub mod list;
pub mod patch;

pub use batch_update::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest,
};

pub use children::{
    BatchDeleteDocumentBlockChildrenParams, BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse, CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest, CreateDocumentBlockChildrenResponse,
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
};

pub use descendant::{
    BlockIdRelation, CreateDocumentBlockDescendantParams, CreateDocumentBlockDescendantRequest,
    CreateDocumentBlockDescendantResponse,
};

pub use get::{
    GetDocumentBlockParams, GetDocumentBlockRequest, GetDocumentBlockResponse,
};

pub use list::{
    GetDocumentBlocksParams, GetDocumentBlocksRequest, GetDocumentBlocksResponse,
};

pub use patch::{
    UpdateDocumentBlockParams, UpdateDocumentBlockRequest, UpdateDocumentBlockResponse,
};
