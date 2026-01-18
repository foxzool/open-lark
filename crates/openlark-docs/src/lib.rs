#![warn(clippy::all)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(ambiguous_glob_reexports)]
#![allow(hidden_glob_reexports)]
// #![deny(missing_docs)] // 暂时禁用，在开发阶段

//! # OpenLark 文档服务模块
//!
//! 飞书开放平台云文档服务模块，提供文档、表格、知识库等 API 访问能力。
//!
//! ## CCM业务域模块结构 (严格按照project-version-resource组织)
//!
//! - **ccm**: 云内容管理（174 APIs）- 文档、表格、知识库、云盘
//!   - **ccm_doc**: 旧版文档（6 APIs）
//!   - **ccm_docs**: 云文档管理（2 APIs）
//!   - **ccm_drive_explorer**: 云盘浏览器（8 APIs）
//!   - **ccm_drive_permission**: 云文档权限（3 APIs）
//!   - **ccm_sheet**: 表格（33 APIs）
//!   - **docs**: 文档内容（1 API）
//!   - **docx**: 新版文档块与群公告（19 APIs）
//!   - **drive**: 云空间文件（59 APIs）
//!   - **sheets**: 表格（27 APIs）
//!   - **wiki**: Wiki（16 APIs）
//! - **base**: 基础服务（3 APIs）
//! - **bitable**: 多维表格（46 APIs）
//! - **baike**: 百科（13 APIs）
//! - **lingo**: 词典（14 APIs）
//! - **minutes**: 妙记（4 APIs）
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! use openlark_core::config::Config;
//! use openlark_docs::DocsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .build();
//!     let client = DocsClient::new(config);
//!
//!     // 使用链式调用访问云盘文件服务
//!     // client.ccm().drive().v1().file()...
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 特性
//!
//! - ✅ **254 APIs 全覆盖** - 飞书云文档服务完整实现
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于 tokio 的异步 API
//! - ✅ **版本化 API** - 支持 v1/v2/v3/v4 多版本 API
//! - ✅ **构建器模式** - 流畅的 API 调用体验
//! - ✅ **标准架构** - 严格按照 bizTag/project/version/resource/name.rs 模式组织

// Include macros first - 对所有功能都启用宏
#[macro_use]
mod macros;

// Core modules
pub mod error;
pub mod models;
pub mod service;

// 功能模块按业务域组织
#[cfg(feature = "ccm-core")]
pub mod ccm;

#[cfg(any(feature = "base", feature = "bitable"))]
pub mod base;

#[cfg(any(feature = "baike", feature = "lingo"))]
pub mod baike;

#[cfg(feature = "minutes")]
pub mod minutes;

// === 兼容导出：保持历史模块路径不变 ===
// 注意：实现文件严格按 src/bizTag/project/version/resource/name.rs 组织，
// 这里只做模块别名/重导出，不属于 API 实现文件。
#[cfg(feature = "bitable")]
pub use base::bitable;

#[cfg(feature = "lingo")]
pub use baike::lingo;

#[cfg(feature = "ccm-wiki")]
pub use ccm::wiki;

// docs和docx模块已包含在ccm模块中，无需独立导出

// API版本模块
#[cfg(any(feature = "v1", feature = "v2", feature = "v3", feature = "v4"))]
pub mod versions;

// 通用模块 - 工具宏和类型
pub mod common;

// 端点定义模块
pub mod endpoints;

// Prelude模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use common::chain::DocsClient;
pub use error::{DocsError, DocsResult};
pub use service::DocsService as MainDocsService;

// 重新导出各域服务
#[cfg(feature = "ccm-core")]
pub use ccm::CcmService;

#[cfg(feature = "lingo")]
pub use baike::lingo::LingoService;

#[cfg(feature = "minutes")]
pub use minutes::MinutesService;

#[cfg(feature = "ccm-wiki")]
pub use wiki::WikiService;

// docs和docx服务通过ccm模块导出
// #[cfg(feature = "docs")]
// pub use docs::DocsService;
// #[cfg(feature = "docx")]
// pub use docx::DocxService;
