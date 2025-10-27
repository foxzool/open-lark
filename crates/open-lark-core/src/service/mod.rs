//! Service Module Shim for open-lark-core
//!
//! This module provides temporary shim definitions for service types that
//! were moved to separate crates. This is a transitional solution to maintain
//! compilation compatibility while the multi-crate architecture migration is
//! being completed.


// Re-export shim implementations directly
pub mod shim;
pub use shim::*;

// Direct re-exports from shims for easier access
#[cfg(feature = "contact")]
pub use shim::ContactService;

#[cfg(feature = "im")]
pub use shim::ImService;

#[cfg(feature = "attendance")]
pub use shim::AttendanceService;

#[cfg(feature = "authentication")]
pub use shim::AuthenService;

#[cfg(feature = "acs")]
pub use shim::AcsService;

#[cfg(feature = "admin")]
pub use shim::AdminService;

#[cfg(feature = "ai")]
pub use shim::AiService;

#[cfg(feature = "aily")]
pub use shim::AilyService;

#[cfg(feature = "apass")]
pub use shim::ApassService;

#[cfg(feature = "application")]
pub use shim::ApplicationService;

#[cfg(feature = "approval")]
pub use shim::ApprovalService;

#[cfg(feature = "bot")]
pub use shim::BotService;

#[cfg(feature = "calendar")]
pub use shim::CalendarService;

#[cfg(feature = "cardkit")]
pub use shim::CardkitService;

#[cfg(feature = "corehr")]
pub use shim::CoreHRService;

#[cfg(feature = "directory")]
pub use shim::DirectoryService;

#[cfg(feature = "ehr")]
pub use shim::EhrService;

#[cfg(feature = "elearning")]
pub use shim::ELearningService;

#[cfg(feature = "group")]
pub use shim::GroupService;

#[cfg(feature = "helpdesk")]
pub use shim::HelpdeskService;

#[cfg(feature = "hire")]
pub use shim::HireService;

#[cfg(feature = "human-authentication")]
pub use shim::HumanAuthenticationService;

#[cfg(feature = "lingo")]
pub use shim::LingoService;

#[cfg(feature = "mail")]
pub use shim::MailService;

#[cfg(feature = "mdm")]
pub use shim::MdmService;

#[cfg(feature = "minutes")]
pub use shim::MinutesService;

#[cfg(feature = "moments")]
pub use shim::MomentsService;

#[cfg(feature = "okr")]
pub use shim::OkrService;

#[cfg(feature = "payroll")]
pub use shim::PayrollService;

#[cfg(feature = "performance")]
pub use shim::PerformanceService;

#[cfg(feature = "personal-settings")]
pub use shim::PersonalSettingsService;

#[cfg(feature = "report")]
pub use shim::ReportService;

#[cfg(feature = "search")]
pub use shim::SearchService;

#[cfg(feature = "security-and-compliance")]
pub use shim::SecurityAndComplianceService;

#[cfg(feature = "task")]
pub use shim::TaskV2Service;

#[cfg(feature = "tenant")]
pub use shim::TenantService;

#[cfg(feature = "tenant-tag")]
pub use shim::TenantTagService;

#[cfg(feature = "trust-party")]
pub use shim::TrustPartyService;

#[cfg(feature = "vc")]
pub use shim::VcService;

#[cfg(feature = "verification")]
pub use shim::VerificationService;

#[cfg(feature = "workplace")]
pub use shim::WorkplaceService;

// Cloud docs backward compatibility
#[cfg(feature = "cloud-docs")]
pub use shim::{
    AssistantService, BitableService, BoardService, CommentsService,
    DocsService, DriveService, PermissionService, SheetsService, WikiService,
    CloudDocsService
};