// 核心服务模块
pub mod application;
pub mod approval;
pub mod attendance;
pub mod authentication;
pub mod bot;
pub mod calendar;
pub mod cardkit;
pub mod directory;
pub mod endpoints;
pub mod group;
pub mod helpdesk;
pub mod im;
pub mod mail;
pub mod minutes;
pub mod search;
pub mod task;
pub mod tenant_tag;
pub mod vc;

// 云文档服务模块
pub mod cloud_docs;

// 向后兼容的 re-export
pub use cloud_docs::docx as docs; // docs -> docx 兼容
pub use cloud_docs::{assistant, bitable, board, comments, drive, permission, sheets, wiki};

// 核心服务 re-export
pub use application::ApplicationService;
pub use approval::ApprovalService;
pub use bot::BotService;
pub use calendar::CalendarService;
pub use cardkit::CardkitService;
pub use directory::DirectoryService;
pub use group::GroupService;
pub use helpdesk::HelpdeskService;
pub use mail::MailService;
pub use minutes::MinutesService;
pub use task::TaskV2Service;
pub use tenant_tag::TenantTagService;
pub use vc::VcService;

// 服务类型 re-export
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
