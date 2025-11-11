//! API端点分类模块
//!
//! 按业务功能组织的端点常量集合，提供清晰的分层架构。

pub mod admin;
pub mod ai_services;
pub mod collaboration;
pub mod content;
pub mod hr_management;
pub mod integration;
pub mod messaging;

// 重新导出所有分类
pub use admin::Admin;
pub use ai_services::AiServices;
pub use collaboration::Collaboration;
pub use content::Content;
pub use hr_management::HrManagement;
pub use integration::Integration;
pub use messaging::Messaging;
