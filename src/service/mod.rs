// 核心服务模块
pub mod acs;
pub mod admin;
pub mod ai;
pub mod aily;
pub mod apass;
pub mod application;
pub mod approval;
pub mod attendance;
pub mod authentication;
pub mod bot;
pub mod calendar;
pub mod cardkit;
pub mod corehr;
pub mod directory;
pub mod ehr;
pub mod endpoints;
pub mod group;
pub mod helpdesk;
pub mod hire;
pub mod human_authentication;
pub mod im;
pub mod mail;
pub mod minutes;
pub mod moments;
pub mod okr;
pub mod payroll;
pub mod performance;
pub mod personal_settings;
pub mod search;
pub mod task;
pub mod tenant;
pub mod tenant_tag;
pub mod vc;
pub mod verification;

// 云文档服务模块
pub mod cloud_docs;

// 向后兼容的 re-export
pub use cloud_docs::docx as docs; // docs -> docx 兼容
pub use cloud_docs::{assistant, bitable, board, comments, drive, permission, sheets, wiki};

// 核心服务 re-export
pub use acs::AcsService;
pub use admin::AdminService;
pub use ai::AiService;
pub use aily::AilyService;
pub use apass::ApassService;
pub use application::ApplicationService;
pub use approval::ApprovalService;
pub use bot::BotService;
pub use calendar::CalendarService;
pub use cardkit::CardkitService;
pub use corehr::CoreHRService;
pub use directory::DirectoryService;
pub use ehr::EhrService;
pub use group::GroupService;
pub use helpdesk::HelpdeskService;
pub use hire::HireService;
pub use human_authentication::HumanAuthenticationService;
pub use mail::MailService;
pub use minutes::MinutesService;
pub use moments::MomentsService;
pub use okr::OkrService;
pub use payroll::PayrollService;
pub use performance::PerformanceService;
pub use personal_settings::PersonalSettingsService;
pub use task::TaskV2Service;
pub use tenant::TenantService;
pub use tenant_tag::TenantTagService;
pub use vc::VcService;
pub use verification::VerificationService;

// 服务类型 re-export
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
