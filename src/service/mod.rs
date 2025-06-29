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
pub mod contact;
pub mod corehr;
pub mod directory;
pub mod ehr;
pub mod elearning;
pub mod endpoints;
pub mod group;
pub mod helpdesk;
pub mod hire;
pub mod human_authentication;
pub mod im;
pub mod lingo;
pub mod mail;
pub mod mdm;
pub mod minutes;
pub mod moments;
pub mod okr;
pub mod payroll;
pub mod performance;
pub mod personal_settings;
pub mod report;
pub mod search;
pub mod security_and_compliance;
pub mod task;
pub mod tenant;
pub mod tenant_tag;
pub mod trust_party;
pub mod vc;
pub mod verification;
pub mod workplace;

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
pub use contact::ContactService;
pub use corehr::CoreHRService;
pub use directory::DirectoryService;
pub use ehr::EhrService;
pub use elearning::ELearningService;
pub use group::GroupService;
pub use helpdesk::HelpdeskService;
pub use hire::HireService;
pub use human_authentication::HumanAuthenticationService;
pub use lingo::LingoService;
pub use mail::MailService;
pub use mdm::MdmService;
pub use minutes::MinutesService;
pub use moments::MomentsService;
pub use okr::OkrService;
pub use payroll::PayrollService;
pub use performance::PerformanceService;
pub use personal_settings::PersonalSettingsService;
pub use report::ReportService;
pub use security_and_compliance::SecurityAndComplianceService;
pub use task::TaskV2Service;
pub use tenant::TenantService;
pub use tenant_tag::TenantTagService;
pub use trust_party::TrustPartyService;
pub use vc::VcService;
pub use verification::VerificationService;
pub use workplace::WorkplaceService;

// 服务类型 re-export
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
