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
//! ## 文档模块业务域结构 (严格按照 bizTag/project/version/resource 组织)
//!
//! ### CCM - 云文档协同 (174 APIs)
//! - **docx**: 新版文档块与群公告（19 APIs）
//! - **drive**: 云空间文件管理（59 APIs）
//! - **sheets**: 电子表格（27 APIs）
//! - **wiki**: 知识库（16 APIs）
//! - **doc**: 文档基础服务
//! - **docs**: 云文档管理
//!
//! ### BASE - 基础服务 (49 APIs)
//! - **base**: 基础应用服务
//! - **bitable**: 多维表格（46 APIs）
//!
//! ### BAIKE - 企业知识库 (27 APIs)
//! - **baike**: 知识库空间和节点
//! - **lingo**: 知识库分类和实体
//!
//! ### MINUTES - 会议纪要 (4 APIs)
//! - **minutes**: 会议转录和纪要管理
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
//! - ✅ **202 APIs 全覆盖** - 飞书云文档服务完整实现（不含旧版本）
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于 tokio 的异步 API
//! - ✅ **版本化 API** - 支持 v1/v2/v3/v4 多版本 API
//! - ✅ **构建器模式** - 流畅的 API 调用体验
//! - ✅ **标准架构** - 严格按照 bizTag/project/version/resource/name.rs 模式组织

// Include macros first - 对所有功能都启用宏
#[macro_use]
mod macros;

// Core modules
pub mod models;

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
#[cfg(any(feature = "v1", feature = "v2", feature = "v3"))]
pub mod versions;

// 通用模块 - 工具宏和类型
pub mod common;

// Prelude模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use common::chain::DocsClient;

// 已移除中间 Service 的 public 导出，统一使用 DocsClient 作为唯一入口
// 移除的导出：
// - CcmService（通过 docs.ccm 访问）
// - BaseService（通过 docs.base 访问）
// - BitableService（通过 docs.base.bitable 访问）
// - BaikeService（通过 docs.baike 访问）
// - LingoService（通过 docs.baike.lingo 访问）
// - MinutesService（通过 docs.minutes 访问）
// - WikiService（通过 docs.ccm.wiki 访问）
// - DocsService（通过 docs.ccm.docs 访问）
// - DocxService（通过 docs.ccm.docx 访问）
//
// 注意：Service 类型仍然保留，但不从 lib.rs 导出。如有需要，可通过完整路径访问。
