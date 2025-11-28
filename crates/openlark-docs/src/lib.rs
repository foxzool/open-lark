//! # OpenLark 文档服务模块
//!
//! 飞书开放平台云文档服务模块，提供文档、表格、知识库等API访问能力。
//!
//! ## 功能模块 (按meta.Project组织)
//!
//! ### 核心文档服务
//! - **drive**: 云盘文件管理 (59 APIs) - 文件上传下载、权限控制
//! - **sheets**: 电子表格服务 (27 APIs) - 表格数据操作、工作表管理
//! - **docx**: 文档处理服务 (19 APIs) - 文档创建编辑、内容管理
//!
//! ### 知识库服务
//! - **wiki**: 知识库管理 (16 APIs) - 空间节点管理、协作编辑
//! - **baike**: 知识百科 (13 APIs) - 企业知识库、条目管理
//! - **lingo**: 语言服务 (14 APIs) - 智能语言处理、知识搜索
//!
//! ### 数据管理服务
//! - **bitable**: 多维表格 (46 APIs) - 应用管理、数据表、视图
//! - **base**: 基础服务 (3 APIs) - 权限管理、基础配置
//!
//! ### 会议服务
//! - **minutes**: 会议纪要 (4 APIs) - 会议记录、转录服务
//!
//! ## 快速开始
//!
//! ```rust
//! use openlark_docs::DocsService;
//! use openlark_core::config::Config;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let docs = DocsService::new(config);
//!
//! // 云盘文件服务
//! let files = docs.drive.v1.files();
//! let file_list = files.list().parent_folder_token("token").send().await?;
//!
//! // 多维表格服务
//! let apps = docs.bitable.v1.apps();
//! let app_list = apps.list().send().await?;
//! ```
//!
//! ## 特性
//!
//! - ✅ **254 APIs全覆盖** - 飞书云文档服务完整实现
//! - ✅ **Project-Version-Resource架构** - 清晰的三层结构
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于tokio的异步API
//! - ✅ **版本化API** - 支持v1/v2/v3多版本API
//! - ✅ **构建器模式** - 流畅的API调用体验

// #![deny(missing_docs)] // 暂时禁用，在开发阶段
#![warn(clippy::all)]

// Core modules
pub mod error;
pub mod models;
pub mod prelude;
pub mod service;

// Project-level modules (meta.Project作为一级目录)
pub mod baike;
pub mod base;
pub mod bitable;
pub mod docx;
pub mod drive;
pub mod lingo;
pub mod minutes;
pub mod sheets;
pub mod wiki;

// 测试工具模块（仅在测试时可用）
#[cfg(test)]
pub mod testing;

// 重新导出主要类型
pub use error::{DocsError, DocsResult};
pub use service::DocsService;

// 重新导出各Project服务
pub use baike::BaikeService;
pub use base::BaseService;
pub use bitable::BitableService;
pub use docx::DocxService;
pub use drive::DriveService;
pub use lingo::LingoService;
pub use minutes::MinutesService;
pub use sheets::SheetsService;
pub use wiki::WikiService;
