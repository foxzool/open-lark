//! OpenLark Docs - 飞书云文档服务SDK
//!
//! 提供对飞书云文档、电子表格、文档、Wiki等服务的类型安全访问
//!
//! ## CCM业务域模块结构 (严格按照project-version-resource组织)
//!
//! ### 核心文档服务
//! - **drive**: 云空间文件管理 (59个API) - 文件上传下载、权限控制、元数据管理
//! - **sheets**: 电子表格管理 (60个API) - ccm_sheet(33个) + sheets(27个) 表格数据操作、工作表管理
//! - **docx**: 云文档管理 (19个API) - 文档创建编辑、内容管理、群公告
//!
//! ### 知识库服务
//! - **wiki**: 知识库管理 (16个API) - 空间节点管理、成员管理、设置更新
//!
//! ### CCM专用服务
//! - **ccm_doc**: 文档元数据 (6个API) - 文档基础信息和内容操作
//! - **ccm_docs**: 文档搜索 (2个API) - 文档搜索和元数据获取
//! - **ccm_drive_permission**: 驱动权限 (3个API) - 公开权限和成员权限管理
//! - **docs**: 基础文档服务 (1个API) - 文档内容获取
//!
//! ## 快速开始
//!
//! ```rust
//! use openlark_docs::DocsClient;
//! use openlark_core::config::Config;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let client = DocsClient::new(config);
//!
//! // 云盘文件服务
//! let files = client.drive.v1.file();
//! let folder = files.create_folder("parent_token", "新文件夹").await?;
//!
//! // 电子表格服务
//! let sheets = client.sheets.v2.spreadsheet();
//! let props = sheets.properties("spreadsheet_token").await?;
//! ```
//!
//! ## 特性
//!
//! - ✅ **174个CCM API全覆盖** - 飞书云文档CCM业务域完整实现
//! - ✅ **严格的Project-Version-Resource架构** - 完全按照CSV规范组织
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于tokio的异步API
//! - ✅ **版本化API** - 支持v1/v2/v3多版本API

#![warn(clippy::all)]

// Core modules
pub mod error;
pub mod service;

// 按 CSV meta.Project 组织的模块
pub mod drive;         // 59 (ccm)
pub mod sheets;        // 27 (ccm)
pub mod docx;          // 19 (ccm)
pub mod wiki;          // 16 (ccm)
pub mod ccm_doc;       // 6  (ccm)
pub mod ccm_docs;      // 2  (ccm)
pub mod ccm_drive_permission; // 3 (ccm)
pub mod ccm_drive_explorer;   // 8 (ccm)
pub mod ccm_sheet;            // 33 (ccm_sheet old)
pub mod docs;          // 1  (ccm)
pub mod bitable;       // 46 (base bizTag -> bitable project)
pub mod base;          // 3  (base)
pub mod lingo;         // 14 (baike bizTag)
pub mod baike;         // 13 (baike)
pub mod minutes;       // 4  (minutes)

// 重新导出核心类型
pub use error::{DocsError, DocsResult};
pub use service::DocsService;

// 重新导出SDK核心类型
pub use openlark_core::{config::Config, SDKResult};

use std::sync::Arc;

/// DocsClient：统一入口，提供project-version-resource链式访问
#[derive(Clone)]
pub struct DocsClient {
    service: Arc<DocsService>,
}

impl DocsClient {
    /// 从配置创建文档客户端
    pub fn new(config: Config) -> Self {
        Self {
            service: Arc::new(DocsService::new(config)),
        }
    }

    /// 访问 drive 项目
    pub fn drive(&self) -> drive::Drive {
        drive::Drive::new(self.service.clone())
    }

    /// 访问 sheets 项目 (v3)
    pub fn sheets(&self) -> sheets::Sheets {
        sheets::Sheets::new(self.service.clone())
    }

    /// 访问 docx 项目
    pub fn docx(&self) -> docx::Docx {
        docx::Docx::new(self.service.clone())
    }

    /// 访问 wiki 项目
    pub fn wiki(&self) -> wiki::Wiki {
        wiki::Wiki::new(self.service.clone())
    }

    /// 访问 ccm_doc 旧版文档元数据
    pub fn ccm_doc(&self) -> ccm_doc::CcmDoc {
        ccm_doc::CcmDoc::new(self.service.clone())
    }

    /// 访问 ccm_docs 搜索
    pub fn ccm_docs(&self) -> ccm_docs::CcmDocs {
        ccm_docs::CcmDocs::new(self.service.clone())
    }

    /// 访问 ccm_drive_permission 公开/成员权限
    pub fn ccm_drive_permission(&self) -> ccm_drive_permission::CcmDrivePermission {
        ccm_drive_permission::CcmDrivePermission::new(self.service.clone())
    }

    /// 访问 ccm_drive_explorer (旧版云盘浏览)
    pub fn ccm_drive_explorer(&self) -> ccm_drive_explorer::CcmDriveExplorer {
        ccm_drive_explorer::CcmDriveExplorer::new(self.service.clone())
    }

    /// 访问 ccm_sheet (旧版表格)
    pub fn ccm_sheet(&self) -> ccm_sheet::CcmSheet {
        ccm_sheet::CcmSheet::new(self.service.clone())
    }

    /// 访问 docs 基础内容
    pub fn docs(&self) -> docs::Docs {
        docs::Docs::new(self.service.clone())
    }

    /// 访问 bitable (base 业务)
    pub fn bitable(&self) -> bitable::Bitable {
        bitable::Bitable::new(self.service.clone())
    }

    /// 访问 base (权限增强 3 API)
    pub fn base(&self) -> base::Base {
        base::Base::new(self.service.clone())
    }

    /// 访问 lingo (新版知识库)
    pub fn lingo(&self) -> lingo::Lingo {
        lingo::Lingo::new(self.service.clone())
    }

    /// 访问 baike (旧版知识库)
    pub fn baike(&self) -> baike::Baike {
        baike::Baike::new(self.service.clone())
    }

    /// 访问 minutes (会议纪要)
    pub fn minutes(&self) -> minutes::Minutes {
        minutes::Minutes::new(self.service.clone())
    }
}
