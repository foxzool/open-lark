//! API端点分类模块
//!
//! 按业务功能组织的端点常量集合，提供清晰的分层架构。

pub mod messaging;
pub mod content;
pub mod hr_management;
pub mod collaboration;
pub mod ai_services;
pub mod admin;
pub mod integration;

// 重新导出所有分类
pub use messaging::Messaging;
pub use content::Content;
pub use hr_management::HrManagement;
pub use collaboration::Collaboration;
pub use ai_services::AiServices;
pub use admin::Admin;
pub use integration::Integration;