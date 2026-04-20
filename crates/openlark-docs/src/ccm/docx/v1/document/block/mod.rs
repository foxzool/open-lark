/// block模块 - 文档块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_update;
/// children 子模块。
pub mod children;
/// descendant 子模块。
pub mod descendant;
/// get 子模块。
pub mod get;
/// list 子模块。
pub mod list;
/// patch 子模块。
pub mod patch;

/// 重新导出相关类型。
pub use batch_update::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest,
};

/// 重新导出相关类型。
pub use children::{
    BatchDeleteDocumentBlockChildrenParams, BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse, CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest, CreateDocumentBlockChildrenResponse,
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
};

/// 重新导出相关类型。
pub use descendant::{
    BlockIdRelation, CreateDocumentBlockDescendantParams, CreateDocumentBlockDescendantRequest,
    CreateDocumentBlockDescendantResponse,
};

/// 重新导出相关类型。
pub use get::{GetDocumentBlockParams, GetDocumentBlockRequest, GetDocumentBlockResponse};

/// 重新导出相关类型。
pub use list::{GetDocumentBlocksParams, GetDocumentBlocksRequest, GetDocumentBlocksResponse};

/// 重新导出相关类型。
pub use patch::{
    UpdateDocumentBlockParams, UpdateDocumentBlockRequest, UpdateDocumentBlockResponse,
};
