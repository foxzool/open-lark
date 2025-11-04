//! Service Shim Implementations
//!
//! Temporary shim implementations for services during the migration period.

// Imports for service implementations (only when features are enabled)
#[cfg(any(
    feature = "contact",
    feature = "im",
    feature = "attendance",
    feature = "authentication",
    feature = "bot",
    feature = "calendar",
    feature = "cardkit",
    feature = "cloud-docs",
    feature = "corehr",
    feature = "directory",
    feature = "ehr",
    feature = "elearning",
    feature = "group",
    feature = "helpdesk",
    feature = "hire",
    feature = "human-authentication",
    feature = "lingo",
    feature = "mail",
    feature = "mdm",
    feature = "minutes",
    feature = "moments",
    feature = "okr",
    feature = "payroll",
    feature = "performance",
    feature = "personal-settings",
    feature = "report",
    feature = "search",
    feature = "security-and-compliance",
    feature = "task",
    feature = "tenant",
    feature = "tenant-tag",
    feature = "trust-party",
    feature = "vc",
    feature = "verification",
    feature = "workplace"
))]
use crate::core::config::Config;
#[cfg(any(
    feature = "contact",
    feature = "im",
    feature = "attendance",
    feature = "authentication",
    feature = "bot",
    feature = "calendar",
    feature = "cardkit",
    feature = "cloud-docs",
    feature = "corehr",
    feature = "directory",
    feature = "ehr",
    feature = "elearning",
    feature = "group",
    feature = "helpdesk",
    feature = "hire",
    feature = "human-authentication",
    feature = "lingo",
    feature = "mail",
    feature = "mdm",
    feature = "minutes",
    feature = "moments",
    feature = "okr",
    feature = "payroll",
    feature = "performance",
    feature = "personal-settings",
    feature = "report",
    feature = "search",
    feature = "security-and-compliance",
    feature = "task",
    feature = "tenant",
    feature = "tenant-tag",
    feature = "trust-party",
    feature = "vc",
    feature = "verification",
    feature = "workplace"
))]
use std::sync::Arc;

// Include additional services
pub mod additional_services;
#[cfg(any(
    feature = "acs",
    feature = "admin",
    feature = "ai",
    feature = "aily",
    feature = "apass",
    feature = "application",
    feature = "approval",
    feature = "payroll",
    feature = "cloud-docs"
))]
pub use additional_services::*;

/// Contact Service Shim
///
/// Temporary implementation for ContactService during migration period.
/// This will be replaced by the full implementation from the
/// open-lark-communication crate once the architecture is stabilized.
#[cfg(feature = "contact")]
pub struct ContactService {
    _config: Config,
}

#[cfg(feature = "contact")]
impl ContactService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "contact")]
impl Clone for ContactService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "contact")]
impl std::fmt::Debug for ContactService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContactService")
            .field("migrated", &true)
            .finish()
    }
}

/// IM Service Shim
///
/// Temporary implementation for ImService during migration period.
/// This will be replaced by the full implementation from the
/// open-lark-communication crate once the architecture is stabilized.
#[cfg(feature = "im")]
pub struct ImService {
    _config: Config,
}

#[cfg(feature = "im")]
impl ImService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "im")]
impl Clone for ImService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "im")]
impl std::fmt::Debug for ImService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImService")
            .field("migrated", &true)
            .finish()
    }
}

// Additional service shims
#[cfg(feature = "attendance")]
pub struct AttendanceService {
    _config: Config,
}

#[cfg(feature = "attendance")]
impl AttendanceService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "attendance")]
impl Clone for AttendanceService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "attendance")]
impl std::fmt::Debug for AttendanceService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttendanceService")
            .field("migrated", &true)
            .finish()
    }
}

#[cfg(feature = "authentication")]
pub struct AuthenService {
    _config: Config,
}

#[cfg(feature = "authentication")]
impl AuthenService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "authentication")]
impl Clone for AuthenService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "authentication")]
impl std::fmt::Debug for AuthenService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthenService")
            .field("migrated", &true)
            .finish()
    }
}

#[cfg(feature = "bot")]
pub struct BotService {
    _config: Config,
}

#[cfg(feature = "bot")]
impl BotService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "bot")]
impl Clone for BotService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "bot")]
impl std::fmt::Debug for BotService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BotService")
            .field("migrated", &true)
            .finish()
    }
}

#[cfg(feature = "calendar")]
pub struct CalendarService {
    _config: Config,
}

#[cfg(feature = "calendar")]
impl CalendarService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "calendar")]
impl Clone for CalendarService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "calendar")]
impl std::fmt::Debug for CalendarService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CalendarService")
            .field("migrated", &true)
            .finish()
    }
}

#[cfg(feature = "cardkit")]
pub struct CardkitService {
    _config: Config,
}

#[cfg(feature = "cardkit")]
impl CardkitService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "cardkit")]
impl Clone for CardkitService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "cardkit")]
impl std::fmt::Debug for CardkitService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardkitService")
            .field("migrated", &true)
            .finish()
    }
}

#[cfg(feature = "cloud-docs")]
pub struct CloudDocsService {
    _config: Config,
}

#[cfg(feature = "cloud-docs")]
impl CloudDocsService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "cloud-docs")]
impl Clone for CloudDocsService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "cloud-docs")]
impl std::fmt::Debug for CloudDocsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloudDocsService")
            .field("migrated", &true)
            .finish()
    }
}

// CoreHR Service Shim
#[cfg(feature = "corehr")]
pub struct CoreHRService {
    _config: Config,
}

#[cfg(feature = "corehr")]
impl CoreHRService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "corehr")]
impl Clone for CoreHRService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "corehr")]
impl std::fmt::Debug for CoreHRService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreHRService")
            .field("migrated", &true)
            .finish()
    }
}

// Directory Service Shim
#[cfg(feature = "directory")]
pub struct DirectoryService {
    _config: Config,
}

#[cfg(feature = "directory")]
impl DirectoryService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "directory")]
impl Clone for DirectoryService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "directory")]
impl std::fmt::Debug for DirectoryService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectoryService")
            .field("migrated", &true)
            .finish()
    }
}

// EHR Service Shim
#[cfg(feature = "ehr")]
pub struct EhrService {
    _config: Config,
}

#[cfg(feature = "ehr")]
impl EhrService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "ehr")]
impl Clone for EhrService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "ehr")]
impl std::fmt::Debug for EhrService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EhrService")
            .field("migrated", &true)
            .finish()
    }
}

// ELearning Service Shim
#[cfg(feature = "elearning")]
pub struct ELearningService {
    _config: Config,
}

#[cfg(feature = "elearning")]
impl ELearningService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "elearning")]
impl Clone for ELearningService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "elearning")]
impl std::fmt::Debug for ELearningService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ELearningService")
            .field("migrated", &true)
            .finish()
    }
}

// Group Service Shim
#[cfg(feature = "group")]
pub struct GroupService {
    _config: Config,
}

#[cfg(feature = "group")]
impl GroupService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "group")]
impl Clone for GroupService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "group")]
impl std::fmt::Debug for GroupService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupService")
            .field("migrated", &true)
            .finish()
    }
}

// Helpdesk Service Shim
#[cfg(feature = "helpdesk")]
pub struct HelpdeskService {
    _config: Config,
}

#[cfg(feature = "helpdesk")]
impl HelpdeskService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "helpdesk")]
impl Clone for HelpdeskService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "helpdesk")]
impl std::fmt::Debug for HelpdeskService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelpdeskService")
            .field("migrated", &true)
            .finish()
    }
}

// Hire Service Shim
#[cfg(feature = "hire")]
pub struct HireService {
    _config: Config,
}

#[cfg(feature = "hire")]
impl HireService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "hire")]
impl Clone for HireService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "hire")]
impl std::fmt::Debug for HireService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HireService")
            .field("migrated", &true)
            .finish()
    }
}

// Human Authentication Service Shim
#[cfg(feature = "human-authentication")]
pub struct HumanAuthenticationService {
    _config: Config,
}

#[cfg(feature = "human-authentication")]
impl HumanAuthenticationService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "human-authentication")]
impl Clone for HumanAuthenticationService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "human-authentication")]
impl std::fmt::Debug for HumanAuthenticationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HumanAuthenticationService")
            .field("migrated", &true)
            .finish()
    }
}

// Lingo Service Shim
#[cfg(feature = "lingo")]
pub struct LingoService {
    _config: Config,
}

#[cfg(feature = "lingo")]
impl LingoService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "lingo")]
impl Clone for LingoService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "lingo")]
impl std::fmt::Debug for LingoService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LingoService")
            .field("migrated", &true)
            .finish()
    }
}

// Mail Service Shim
#[cfg(feature = "mail")]
pub struct MailService {
    _config: Config,
}

#[cfg(feature = "mail")]
impl MailService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "mail")]
impl Clone for MailService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "mail")]
impl std::fmt::Debug for MailService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MailService")
            .field("migrated", &true)
            .finish()
    }
}

// MDM Service Shim
#[cfg(feature = "mdm")]
pub struct MdmService {
    _config: Config,
}

#[cfg(feature = "mdm")]
impl MdmService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "mdm")]
impl Clone for MdmService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "mdm")]
impl std::fmt::Debug for MdmService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdmService")
            .field("migrated", &true)
            .finish()
    }
}

// Minutes Service Shim
#[cfg(feature = "minutes")]
pub struct MinutesService {
    _config: Config,
}

#[cfg(feature = "minutes")]
impl MinutesService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "minutes")]
impl Clone for MinutesService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "minutes")]
impl std::fmt::Debug for MinutesService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinutesService")
            .field("migrated", &true)
            .finish()
    }
}

// Moments Service Shim
#[cfg(feature = "moments")]
pub struct MomentsService {
    _config: Config,
}

#[cfg(feature = "moments")]
impl MomentsService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "moments")]
impl Clone for MomentsService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "moments")]
impl std::fmt::Debug for MomentsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MomentsService")
            .field("migrated", &true)
            .finish()
    }
}

// OKR Service Shim
#[cfg(feature = "okr")]
pub struct OkrService {
    _config: Config,
}

#[cfg(feature = "okr")]
impl OkrService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "okr")]
impl Clone for OkrService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "okr")]
impl std::fmt::Debug for OkrService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OkrService")
            .field("migrated", &true)
            .finish()
    }
}

// Payroll Service Shim
#[cfg(feature = "payroll")]
pub struct PayrollService {
    _config: Config,
}

#[cfg(feature = "payroll")]
impl PayrollService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "payroll")]
impl Clone for PayrollService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "payroll")]
impl std::fmt::Debug for PayrollService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PayrollService")
            .field("migrated", &true)
            .finish()
    }
}

// Performance Service Shim
#[cfg(feature = "performance")]
pub struct PerformanceService {
    _config: Config,
}

#[cfg(feature = "performance")]
impl PerformanceService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "performance")]
impl Clone for PerformanceService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "performance")]
impl std::fmt::Debug for PerformanceService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerformanceService")
            .field("migrated", &true)
            .finish()
    }
}

// Personal Settings Service Shim
#[cfg(feature = "personal-settings")]
pub struct PersonalSettingsService {
    _config: Config,
}

#[cfg(feature = "personal-settings")]
impl PersonalSettingsService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "personal-settings")]
impl Clone for PersonalSettingsService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "personal-settings")]
impl std::fmt::Debug for PersonalSettingsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersonalSettingsService")
            .field("migrated", &true)
            .finish()
    }
}

// Report Service Shim
#[cfg(feature = "report")]
pub struct ReportService {
    _config: Config,
}

#[cfg(feature = "report")]
impl ReportService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "report")]
impl Clone for ReportService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "report")]
impl std::fmt::Debug for ReportService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReportService")
            .field("migrated", &true)
            .finish()
    }
}

// Search Service Shim
#[cfg(feature = "search")]
pub struct SearchService {
    _config: Config,
}

#[cfg(feature = "search")]
impl SearchService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "search")]
impl Clone for SearchService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "search")]
impl std::fmt::Debug for SearchService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchService")
            .field("migrated", &true)
            .finish()
    }
}

// Security And Compliance Service Shim
#[cfg(feature = "security-and-compliance")]
pub struct SecurityAndComplianceService {
    _config: Config,
}

#[cfg(feature = "security-and-compliance")]
impl SecurityAndComplianceService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "security-and-compliance")]
impl Clone for SecurityAndComplianceService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "security-and-compliance")]
impl std::fmt::Debug for SecurityAndComplianceService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecurityAndComplianceService")
            .field("migrated", &true)
            .finish()
    }
}

// Task V2 Service Shim
#[cfg(feature = "task")]
pub struct TaskV2Service {
    _config: Config,
}

#[cfg(feature = "task")]
impl TaskV2Service {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "task")]
impl Clone for TaskV2Service {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "task")]
impl std::fmt::Debug for TaskV2Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskV2Service")
            .field("migrated", &true)
            .finish()
    }
}

// Tenant Service Shim
#[cfg(feature = "tenant")]
pub struct TenantService {
    _config: Config,
}

#[cfg(feature = "tenant")]
impl TenantService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "tenant")]
impl Clone for TenantService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "tenant")]
impl std::fmt::Debug for TenantService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TenantService")
            .field("migrated", &true)
            .finish()
    }
}

// Tenant Tag Service Shim
#[cfg(feature = "tenant-tag")]
pub struct TenantTagService {
    _config: Config,
}

#[cfg(feature = "tenant-tag")]
impl TenantTagService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "tenant-tag")]
impl Clone for TenantTagService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "tenant-tag")]
impl std::fmt::Debug for TenantTagService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TenantTagService")
            .field("migrated", &true)
            .finish()
    }
}

// Trust Party Service Shim
#[cfg(feature = "trust-party")]
pub struct TrustPartyService {
    _config: Config,
}

#[cfg(feature = "trust-party")]
impl TrustPartyService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "trust-party")]
impl Clone for TrustPartyService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "trust-party")]
impl std::fmt::Debug for TrustPartyService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrustPartyService")
            .field("migrated", &true)
            .finish()
    }
}

// VC Service Shim
#[cfg(feature = "vc")]
pub struct VcService {
    _config: Config,
}

#[cfg(feature = "vc")]
impl VcService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "vc")]
impl Clone for VcService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "vc")]
impl std::fmt::Debug for VcService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VcService")
            .field("migrated", &true)
            .finish()
    }
}

// Verification Service Shim
#[cfg(feature = "verification")]
pub struct VerificationService {
    _config: Config,
}

#[cfg(feature = "verification")]
impl VerificationService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "verification")]
impl Clone for VerificationService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "verification")]
impl std::fmt::Debug for VerificationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerificationService")
            .field("migrated", &true)
            .finish()
    }
}

// Workplace Service Shim
#[cfg(feature = "workplace")]
pub struct WorkplaceService {
    _config: Config,
}

#[cfg(feature = "workplace")]
impl WorkplaceService {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self {
            _config: shared_config.as_ref().clone(),
        }
    }
}

#[cfg(feature = "workplace")]
impl Clone for WorkplaceService {
    fn clone(&self) -> Self {
        Self {
            _config: self._config.clone(),
        }
    }
}

#[cfg(feature = "workplace")]
impl std::fmt::Debug for WorkplaceService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkplaceService")
            .field("migrated", &true)
            .finish()
    }
}

// Add remaining service shims as needed
// This will be expanded based on testing requirements
