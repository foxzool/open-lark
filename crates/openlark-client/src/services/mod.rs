//! OpenLark Client 服务访问层
//!
//! 提供统一的服务访问接口，作为底层crates的薄包装层

// 业务服务模块



/// 认证服务模块
///
/// 提供飞书认证和授权功能，包括令牌管理、OAuth验证等
#[cfg(feature = "auth")]
pub mod auth;


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


// 重新导出所有服务类型

#[cfg(feature = "auth")]
pub use auth::AuthService;


#[cfg(feature = "communication")]
pub use communication::CommunicationService;

#[cfg(feature = "docs")]
pub use docs::DocsService;


#[cfg(feature = "hr")]
pub use hr::HRService;

#[cfg(feature = "ai")]
pub use ai::AIService;


/// 📦 服务访问层预导出
pub mod prelude {
    
    #[cfg(feature = "auth")]
    pub use super::AuthService;

    
    #[cfg(feature = "communication")]
    pub use super::CommunicationService;

    #[cfg(feature = "docs")]
    pub use super::DocsService;

    
    #[cfg(feature = "hr")]
    pub use super::HRService;

    #[cfg(feature = "ai")]
    pub use super::AIService;

    }
