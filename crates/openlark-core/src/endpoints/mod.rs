//! API端点常量定义模块 (重构版)
//!
//! 本模块包含飞书开放平台的核心API端点常量。重构后采用分层架构：
//! - 核心端点保留在 openlark-core 中
//! - 业务服务端点迁移到对应的 service crate 中
//!
//! # 核心设计原则
//!
//! 1. **单一职责**: openlark-core 只包含基础设施相关的端点
//! 2. **模块化**: 业务端点按服务域分离到对应 crate
//! 3. **向后兼容**: 通过重新导出保持 API 兼容性
//! 4. **功能标志**: 支持按需编译和服务组合
//!
//! # 核心端点保留原则
//!
//! 以下类型的端点保留在 core 中：
//! - 基础认证和授权 (auth)
//! - 应用管理 (application)
//! - 平台集成 (platform_integration)
//! - 通用基础设施 (apass)
//!
//! # 迁移说明
//!
//! 业务服务端点已迁移到对应 crate：
//! - `openlark-admin`: admin, acs, mdm, tenant, workplace 等
//! - `openlark-ai`: ai, aily, ai_embedding, ai_workflow 等
//! - `openlark-comm`: im, mail, vc 等
//! - `openlark-docs`: cloud_docs, drive, cardkit 等
//! - 其他服务以此类推

// 核心基础端点模块（仅保留基础设施相关的端点）
pub mod auth;
pub mod application;
pub mod apass;
pub mod platform_integration;

// 已迁移到对应 service crate 的端点：
// - approval -> openlark-approval
// - ai, aily -> openlark-ai
// - im, mail, vc -> openlark-comm
// - contact, directory -> openlark-people
// - attendance, corehr -> openlark-hr
// - cloud_docs, drive -> openlark-docs
// - 等等...

// 向后兼容性支持
pub use super::endpoints_original::Endpoints;
pub use super::endpoints_original::EndpointBuilder;

// 导出核心端点常量
pub use auth::*;
pub use application::*;
pub use apass::*;
pub use platform_integration::*;

// 功能标志导出（按需启用）
#[cfg(feature = "admin")]
pub use crate::endpoints_original::admin::*;
