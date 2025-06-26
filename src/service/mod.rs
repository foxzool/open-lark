// 核心服务模块
pub mod attendance;
pub mod authentication;
pub mod calendar;
pub mod cardkit;
pub mod directory;
pub mod endpoints;
pub mod group;
pub mod im;
pub mod search;

// 云文档服务模块
pub mod cloud_docs;

// 向后兼容的 re-export
pub use cloud_docs::docx as docs; // docs -> docx 兼容
pub use cloud_docs::{assistant, bitable, board, comments, drive, permission, sheets, wiki};

// 核心服务 re-export
pub use calendar::CalendarService;
pub use cardkit::CardkitService;
pub use directory::DirectoryService;
pub use group::GroupService;

// 服务类型 re-export
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
