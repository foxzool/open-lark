// 核心服务模块
pub mod attendance;
pub mod authentication;
pub mod im;
pub mod search;
pub mod endpoints;

// 云文档服务模块
pub mod cloud_docs;

// 向后兼容的 re-export
pub use cloud_docs::assistant;
pub use cloud_docs::bitable;
pub use cloud_docs::board;
pub use cloud_docs::comments;
pub use cloud_docs::docx as docs;  // docs -> docx 兼容
pub use cloud_docs::drive;
pub use cloud_docs::permission;
pub use cloud_docs::sheets;
pub use cloud_docs::wiki;

// 服务类型 re-export
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
