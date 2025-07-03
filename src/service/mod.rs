//! 服务模块
//!
//! 飞书开放平台API服务的完整实现，提供企业级应用开发所需的所有核心功能。
//! 本模块包含了飞书平台的全部服务能力，支持即时通讯、云文档、视频会议、
//! 人力资源、办公自动化等各个业务领域的API集成。
//!
//! # 服务分类
//!
//! ## 核心通讯服务
//! - [`im`] - 即时通讯服务，支持消息发送、群组管理、机器人等
//! - [`mail`] - 邮箱服务，提供企业邮件管理功能
//! - [`group`] - 群组服务，群组管理和协作功能
//!
//! ## 云文档协作
//! - [`cloud_docs`] - 云文档服务，文档创建、编辑、协作
//! - [`drive`] - 云盘服务，文件存储和管理
//! - [`sheets`] - 电子表格服务，表格数据处理
//!
//! ## 音视频会议
//! - [`vc`] - 视频会议服务，会议管理和控制
//! - [`minutes`] - 妙记服务，会议记录和转写
//!
//! ## 人力资源管理
//! - [`contact`] - 通讯录服务，组织架构和人员管理
//! - [`hire`] - 招聘服务，招聘流程管理
//! - [`attendance`] - 考勤服务，考勤数据管理
//! - [`corehr`] - 人力资源服务，HR管理功能
//! - [`ehr`] - 员工信息服务
//! - [`payroll`] - 薪酬服务，薪资管理
//!
//! ## 办公自动化
//! - [`approval`] - 审批服务，工作流程管理
//! - [`task`] - 任务服务，任务和项目管理
//! - [`okr`] - OKR服务，目标管理
//! - [`calendar`] - 日历服务，日程安排
//!
//! ## 企业管理
//! - [`admin`] - 管理后台服务，企业管理功能
//! - [`tenant`] - 企业信息服务，租户管理
//! - [`directory`] - 组织架构服务，部门员工管理
//! - [`application`] - 应用管理服务，应用生命周期
//!
//! ## 智能化服务
//! - [`ai`] - AI服务，人工智能能力
//! - [`search`] - 搜索服务，企业搜索功能
//! - [`lingo`] - 词典服务，知识管理
//!
//! ## 安全与合规
//! - [`verification`] - 认证服务，身份验证
//! - [`security_and_compliance`] - 安全合规服务
//! - [`apass`] - 通行证服务
//! - [`acs`] - 门禁服务
//!
//! ## 其他服务
//! - [`helpdesk`] - 服务台，客户服务
//! - [`moments`] - 公司圈，企业社交
//! - [`personal_settings`] - 个人设置服务
//! - [`bot`] - 机器人服务
//! - [`cardkit`] - 卡片组件服务
//!
//! # 使用示例
//!
//! ```no_run
//! use open_lark::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建客户端
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 服务调用示例（需要相应的请求对象）
//! // let message = client.im.v1.message.create(message_request, None).await?;
//! // let document = client.cloud_docs.drive.v1.file.create_file(file_request, None).await?;
//! // let employee = client.contact.v3.user.get(user_request, None).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # 设计原则
//!
//! - **统一接口**: 所有服务遵循统一的API设计模式
//! - **版本管理**: 支持多版本API，向后兼容
//! - **类型安全**: 使用Rust类型系统保证API调用安全
//! - **异步支持**: 全面支持异步编程模式
//! - **错误处理**: 统一的错误类型和处理机制

// 核心服务模块 - 使用条件编译
#[cfg(feature = "acs")]
pub mod acs;
#[cfg(feature = "admin")]
pub mod admin;
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "aily")]
pub mod aily;
#[cfg(feature = "apass")]
pub mod apass;
#[cfg(feature = "application")]
pub mod application;
#[cfg(feature = "approval")]
pub mod approval;
#[cfg(feature = "attendance")]
pub mod attendance;
#[cfg(feature = "authentication")]
pub mod authentication;
#[cfg(feature = "bot")]
pub mod bot;
#[cfg(feature = "calendar")]
pub mod calendar;
#[cfg(feature = "cardkit")]
pub mod cardkit;
#[cfg(feature = "contact")]
pub mod contact;
#[cfg(feature = "corehr")]
pub mod corehr;
#[cfg(feature = "directory")]
pub mod directory;
#[cfg(feature = "ehr")]
pub mod ehr;
#[cfg(feature = "elearning")]
pub mod elearning;
pub mod endpoints;
#[cfg(feature = "group")]
pub mod group;
#[cfg(feature = "helpdesk")]
pub mod helpdesk;
#[cfg(feature = "hire")]
pub mod hire;
#[cfg(feature = "human-authentication")]
pub mod human_authentication;
#[cfg(feature = "im")]
pub mod im;
#[cfg(feature = "lingo")]
pub mod lingo;
#[cfg(feature = "mail")]
pub mod mail;
#[cfg(feature = "mdm")]
pub mod mdm;
#[cfg(feature = "minutes")]
pub mod minutes;
#[cfg(feature = "moments")]
pub mod moments;
#[cfg(feature = "okr")]
pub mod okr;
#[cfg(feature = "payroll")]
pub mod payroll;
#[cfg(feature = "performance")]
pub mod performance;
#[cfg(feature = "personal-settings")]
pub mod personal_settings;
#[cfg(feature = "report")]
pub mod report;
#[cfg(feature = "search")]
pub mod search;
#[cfg(feature = "security-and-compliance")]
pub mod security_and_compliance;
#[cfg(feature = "task")]
pub mod task;
#[cfg(feature = "tenant")]
pub mod tenant;
#[cfg(feature = "tenant-tag")]
pub mod tenant_tag;
#[cfg(feature = "trust-party")]
pub mod trust_party;
#[cfg(feature = "vc")]
pub mod vc;
#[cfg(feature = "verification")]
pub mod verification;
#[cfg(feature = "workplace")]
pub mod workplace;

// 云文档服务模块
#[cfg(feature = "cloud-docs")]
pub mod cloud_docs;

// 向后兼容的 re-export - 使用条件编译
#[cfg(feature = "cloud-docs")]
pub use cloud_docs::docx as docs; // docs -> docx 兼容
#[cfg(feature = "cloud-docs")]
pub use cloud_docs::{assistant, bitable, board, comments, drive, permission, sheets, wiki};

// 核心服务 re-export - 使用条件编译
#[cfg(feature = "acs")]
pub use acs::AcsService;
#[cfg(feature = "admin")]
pub use admin::AdminService;
#[cfg(feature = "ai")]
pub use ai::AiService;
#[cfg(feature = "aily")]
pub use aily::AilyService;
#[cfg(feature = "apass")]
pub use apass::ApassService;
#[cfg(feature = "application")]
pub use application::ApplicationService;
#[cfg(feature = "approval")]
pub use approval::ApprovalService;
#[cfg(feature = "bot")]
pub use bot::BotService;
#[cfg(feature = "calendar")]
pub use calendar::CalendarService;
#[cfg(feature = "cardkit")]
pub use cardkit::CardkitService;
#[cfg(feature = "contact")]
pub use contact::ContactService;
#[cfg(feature = "corehr")]
pub use corehr::CoreHRService;
#[cfg(feature = "directory")]
pub use directory::DirectoryService;
#[cfg(feature = "ehr")]
pub use ehr::EhrService;
#[cfg(feature = "elearning")]
pub use elearning::ELearningService;
#[cfg(feature = "group")]
pub use group::GroupService;
#[cfg(feature = "helpdesk")]
pub use helpdesk::HelpdeskService;
#[cfg(feature = "hire")]
pub use hire::HireService;
#[cfg(feature = "human-authentication")]
pub use human_authentication::HumanAuthenticationService;
#[cfg(feature = "lingo")]
pub use lingo::LingoService;
#[cfg(feature = "mail")]
pub use mail::MailService;
#[cfg(feature = "mdm")]
pub use mdm::MdmService;
#[cfg(feature = "minutes")]
pub use minutes::MinutesService;
#[cfg(feature = "moments")]
pub use moments::MomentsService;
#[cfg(feature = "okr")]
pub use okr::OkrService;
#[cfg(feature = "payroll")]
pub use payroll::PayrollService;
#[cfg(feature = "performance")]
pub use performance::PerformanceService;
#[cfg(feature = "personal-settings")]
pub use personal_settings::PersonalSettingsService;
#[cfg(feature = "report")]
pub use report::ReportService;
#[cfg(feature = "search")]
pub use search::SearchService;
#[cfg(feature = "security-and-compliance")]
pub use security_and_compliance::SecurityAndComplianceService;
#[cfg(feature = "task")]
pub use task::TaskV2Service;
#[cfg(feature = "tenant")]
pub use tenant::TenantService;
#[cfg(feature = "tenant-tag")]
pub use tenant_tag::TenantTagService;
#[cfg(feature = "trust-party")]
pub use trust_party::TrustPartyService;
#[cfg(feature = "vc")]
pub use vc::VcService;
#[cfg(feature = "verification")]
pub use verification::VerificationService;
#[cfg(feature = "workplace")]
pub use workplace::WorkplaceService;

// 服务类型 re-export - 使用条件编译
#[cfg(feature = "cloud-docs")]
pub use cloud_docs::{
    AssistantService, BitableService, BoardService, CloudDocsService, CommentsService,
    DocxService as DocsService, DriveService, PermissionService, SheetsService, WikiService,
};
