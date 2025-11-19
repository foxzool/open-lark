//! OpenLark Client 服务访问层
//!
//! 提供统一的服务访问接口，作为底层crates的薄包装层

// 业务服务模块

/// 管理服务模块
///
/// 提供飞书管理和行政功能，包括应用管理、权限控制等
#[cfg(feature = "admin")]
pub mod admin;

/// 审批服务模块
///
/// 提供飞书审批流程管理功能，包括审批创建、查询、处理等
#[cfg(feature = "approval")]
pub mod approval;

/// 认证服务模块
///
/// 提供飞书认证和授权功能，包括令牌管理、OAuth验证等
#[cfg(feature = "auth")]
pub mod auth;

/// 协作服务模块
///
/// 提供飞书协作功能，包括日历、会议、任务管理等
#[cfg(feature = "collab")]
pub mod collab;

/// 通讯服务模块
///
/// 提供飞书通讯功能，包括IM消息、联系人管理、群组管理等
#[cfg(feature = "communication")]
pub mod communication;

/// 文档服务模块
///
/// 提供飞书云文档功能，包括文档、表格、知识库管理等
#[cfg(feature = "docs")]
pub mod docs;

/// 帮助台服务模块
///
/// 提供飞书帮助台功能，包括搜索、百科、技术支持等
#[cfg(feature = "helpdesk")]
pub mod helpdesk;

/// 招聘服务模块
///
/// 提供飞书招聘管理功能，包括职位发布、候选人管理等
#[cfg(feature = "hire")]
pub mod hire;

/// 人力资源服务模块
///
/// 提供飞书人力资源功能，包括员工管理、考勤、薪酬等
#[cfg(feature = "hr")]
pub mod hr;

/// AI服务模块
///
/// 提供飞书AI智能服务，包括智能助手、AI分析等
#[cfg(feature = "ai")]
pub mod ai;

/// 人员服务模块
///
/// 提供飞书人员管理功能，包括联系人、通讯录等
#[cfg(feature = "people")]
pub mod people;

// 重新导出所有服务类型
#[cfg(feature = "admin")]
pub use admin::AdminService;

#[cfg(feature = "approval")]
pub use approval::ApprovalService;

#[cfg(feature = "auth")]
pub use auth::AuthService;

#[cfg(feature = "collab")]
pub use collab::CollabService;

#[cfg(feature = "communication")]
pub use communication::CommunicationService;

#[cfg(feature = "docs")]
pub use docs::DocsService;

#[cfg(feature = "helpdesk")]
pub use helpdesk::HelpdeskService;

#[cfg(feature = "hire")]
pub use hire::HireService;

#[cfg(feature = "hr")]
pub use hr::HRService;

#[cfg(feature = "ai")]
pub use ai::AIService;

#[cfg(feature = "people")]
pub use people::PeopleService;

/// 📦 服务访问层预导出
pub mod prelude {
    #[cfg(feature = "admin")]
    pub use super::AdminService;

    #[cfg(feature = "approval")]
    pub use super::ApprovalService;

    #[cfg(feature = "auth")]
    pub use super::AuthService;

    #[cfg(feature = "collab")]
    pub use super::CollabService;

    #[cfg(feature = "communication")]
    pub use super::CommunicationService;

    #[cfg(feature = "docs")]
    pub use super::DocsService;

    #[cfg(feature = "helpdesk")]
    pub use super::HelpdeskService;

    #[cfg(feature = "hire")]
    pub use super::HireService;

    #[cfg(feature = "hr")]
    pub use super::HRService;

    #[cfg(feature = "ai")]
    pub use super::AIService;

    #[cfg(feature = "people")]
    pub use super::PeopleService;
}
