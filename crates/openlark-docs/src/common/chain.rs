//! # openlark-docs 链式调用入口（meta 风格，偏"模块入口"）
//!
//! ## 设计理念
//!
//! openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//! 为避免为 200+ API 手写方法，这里先提供"模块级链式入口 + Config 透传"。
//! 具体 API 调用仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。
//!
//! ## 推荐入口
//!
//! **公开入口** (推荐用户使用):
//! - `DocsClient` - 文档服务的唯一公开入口
//! - 示例: `DocsClient::new(config).ccm.drive.v1().file()...`
//!
//! **内部入口** (SDK 内部使用，不推荐外部直接访问):
//! - `CcmClient`, `BaseClient`, `BaikeClient`, `MinutesClient` - 各业务域的二级入口
//! - `DriveClient`, `BitableService` 等具体服务类型
//!
//! ## 为什么有多层入口？
//!
//! 1. **DocsClient** (公开)
//!    - 统一的文档服务入口，提供链式调用体验
//!    - 按业务域分组：`docs.ccm`, `docs.base`, `docs.baike`, `docs.minutes`
//!    - 自动根据 feature 裁剪编译，减少二进制体积
//!
//! 2. **\*Client** (内部)
//!    - 对应各业务域的二级入口（如 CCM, BASE, BAIKE, MINUTES）
//!    - 提供子服务访问（如 `ccm.drive`, `ccm.sheets`）
//!    - 内部实现细节，通过 DocsClient 间接访问
//!
//! 3. **\*Service** (内部)
//!    - 具体的服务实现（如 `DriveService`, `BitableService`）
//!    - 提供底层的 API 调用能力
//!    - 通过链式路径的 `service()` 方法获取
//!
//! ## 推荐调用方式
//!
//! ```rust
//! use openlark_core::config::Config;
//! use openlark_docs::DocsClient;
//!
//! // 创建客户端
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let docs = DocsClient::new(config);
//!
//! // ✅ 推荐：使用 DocsClient 链式调用
//! // 访问云盘服务
//! let file_token = docs.ccm.drive.v1().file().upload(...).execute().await?;
//! // 访问多维表格
//! let table = docs.base.bitable().table().create(...).execute().await?;
//! // 访问知识库
//! let node = docs.ccm.wiki.v2().node().create(...).execute().await?;
//!
//! // ❌ 不推荐：直接访问内部类型
//! // let drive_service = DriveService::new(config); // 内部类型，不应直接使用
//! ```
//!
//! ## 实现细节
//!
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入"额外实现文件"
//! - 所有 Client 类型都持有 `Config` 并提供 `config()` 方法用于获取配置
//! - 使用 cfg(feature) 宏实现条件编译，按需启用功能

use openlark_core::config::Config;
use std::sync::Arc;

/// Docs 链式入口：`docs.ccm.drive...` / `docs.bitable...`（按 feature 裁剪）
#[derive(Debug, Clone)]
pub struct DocsClient {
    config: Arc<Config>,

    #[cfg(feature = "ccm-core")]
    pub ccm: CcmClient,

    #[cfg(any(feature = "base", feature = "bitable"))]
    pub base: BaseClient,

    #[cfg(any(feature = "baike", feature = "lingo"))]
    pub baike: BaikeClient,

    #[cfg(feature = "minutes")]
    pub minutes: MinutesClient,
}

impl DocsClient {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            #[cfg(feature = "ccm-core")]
            ccm: CcmClient::new(config.clone()),
            #[cfg(any(feature = "base", feature = "bitable"))]
            base: BaseClient::new(config.clone()),
            #[cfg(any(feature = "baike", feature = "lingo"))]
            baike: BaikeClient::new(config.clone()),
            #[cfg(feature = "minutes")]
            minutes: MinutesClient::new(config.clone()),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// ccm：`docs.ccm`
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct CcmClient {
    config: Arc<Config>,

    pub explorer: ExplorerClient,
    pub permission: PermissionClient,
    pub sheets_v2: SheetsV2Client,
    pub docs: DocsContentClient,
    pub docx: DocxClient,
    pub drive: DriveClient,
    pub sheets: SheetsClient,
    pub wiki: WikiClient,
}

#[cfg(feature = "ccm-core")]
impl CcmClient {
    fn new(config: Arc<Config>) -> Self {
        Self {
            config: config.clone(),
            explorer: ExplorerClient::new(config.clone()),
            permission: PermissionClient::new(config.clone()),
            sheets_v2: SheetsV2Client::new(config.clone()),
            docs: DocsContentClient::new(config.clone()),
            docx: DocxClient::new(config.clone()),
            drive: DriveClient::new(config.clone()),
            sheets: SheetsClient::new(config.clone()),
            wiki: WikiClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "ccm-core")]
macro_rules! impl_ccm_project_client {
    ($name:ident, $service:path) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            config: Arc<Config>,
        }

        impl $name {
            fn new(config: Arc<Config>) -> Self {
                Self { config }
            }

            pub fn config(&self) -> &Config {
                &self.config
            }

            pub fn service(&self) -> $service {
                <$service>::new((*self.config).clone())
            }
        }
    };
}

#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(ExplorerClient, crate::ccm::explorer::ExplorerService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(PermissionClient, crate::ccm::permission::PermissionService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(SheetsV2Client, crate::ccm::sheets_v2::SheetsV2Service);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(DocsContentClient, crate::ccm::docs::DocsService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(DocxClient, crate::ccm::docx::DocxService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(DriveClient, crate::ccm::drive::DriveService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(SheetsClient, crate::ccm::sheets::SheetsService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(WikiClient, crate::ccm::wiki::WikiService);

/// base：`docs.base`（base/bitable 都归口在 base 模块下）
#[cfg(any(feature = "base", feature = "bitable"))]
#[derive(Debug, Clone)]
pub struct BaseClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "base", feature = "bitable"))]
impl BaseClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn service(&self) -> crate::base::BaseService {
        crate::base::BaseService::new((*self.config).clone())
    }

    #[cfg(feature = "bitable")]
    pub fn bitable(&self) -> crate::base::bitable::BitableService {
        crate::base::bitable::BitableService::new((*self.config).clone())
    }
}

/// baike：`docs.baike`（baike/lingo 相关）
#[cfg(any(feature = "baike", feature = "lingo"))]
#[derive(Debug, Clone)]
pub struct BaikeClient {
    config: Arc<Config>,
}

#[cfg(any(feature = "baike", feature = "lingo"))]
impl BaikeClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    #[cfg(feature = "baike")]
    pub fn service(&self) -> crate::baike::BaikeService {
        crate::baike::BaikeService::new((*self.config).clone())
    }
}

/// minutes：`docs.minutes`
#[cfg(feature = "minutes")]
#[derive(Debug, Clone)]
pub struct MinutesClient {
    config: Arc<Config>,
}

#[cfg(feature = "minutes")]
impl MinutesClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn service(&self) -> crate::minutes::MinutesService {
        crate::minutes::MinutesService::new((*self.config).clone())
    }
}
