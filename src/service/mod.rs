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
//! ```rust
//! use open_lark::prelude::*;
//!
//! // 创建客户端
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuilt)
//!     .build();
//!
//! // 使用即时通讯服务
//! let message = client.im.v1.message.create(message_request, None).await?;
//!
//! // 使用云文档服务
//! let document = client.cloud_docs.drive.v1.file.create(file_request, None).await?;
//!
//! // 使用人力资源服务
//! let employee = client.contact.v3.user.get(user_request, None).await?;
//! ```
//!
//! # 设计原则
//!
//! - **统一接口**: 所有服务遵循统一的API设计模式
//! - **版本管理**: 支持多版本API，向后兼容
//! - **类型安全**: 使用Rust类型系统保证API调用安全
//! - **异步支持**: 全面支持异步编程模式
//! - **错误处理**: 统一的错误类型和处理机制

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
