//! API端点常量定义模块
//!
//! 本模块集中定义飞书开放平台的所有API端点路径常量，旨在：
//! 1. 减少字符串分配 - 避免每次API调用时重新创建路径字符串
//! 2. 防止拼写错误 - 统一管理所有API路径，编译期检查
//! 3. 便于维护升级 - 集中管理，方便API版本升级和路径变更
//!
//! # 性能优化
//!
//! 使用静态字符串常量可以显著减少内存分配：
//! ```rust
//! use open_lark::core::endpoints::Endpoints;
//!
//! // 优化前：每次调用时动态分配字符串
//! let dynamic = "/open-apis/workplace/v1/workplace_access_data/search".to_string();
//!
//! // 优化后：使用静态常量，必要时再转换为 String
//! let optimized = Endpoints::WORKPLACE_ACCESS_DATA_SEARCH;
//! assert_eq!(optimized, dynamic.as_str());
//! ```
//!
//! # 组织结构
//!
//! API端点按服务模块分组：
//! - `workplace` - 工作台相关API
//! - `vc` - 视频会议相关API
//! - `im` - 即时消息相关API
//! - `drive` - 云盘相关API
//! - 等等...

// 注意：endpoints_original 现在在 core/mod.rs 中导入，无需重复导入

// 按服务域拆分的模块
pub mod acs;
pub mod admin;
pub mod ai;
pub mod aily;
pub mod apass;
pub mod application;
pub mod approval;
pub mod attendance;
pub mod auth;
pub mod calendar;
pub mod cardkit;
pub mod cloud_docs;
pub mod contact;
pub mod corehr;
pub mod directory;
pub mod drive;
pub mod elearning;
pub mod helpdesk;
pub mod hire;
pub mod im;
pub mod lingo;
pub mod mail;
pub mod mdm;
pub mod minutes;
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

// 重新导出原始端点文件中的所有常量，确保向后兼容性
// 这是一个临时措施，在完成所有域模块迁移后将移除
pub use super::endpoints_original::Endpoints;

// 同时导出 EndpointBuilder (从原始文件)
pub use super::endpoints_original::EndpointBuilder;

// 重新导出各个域模块的常量
pub use lingo::*;
