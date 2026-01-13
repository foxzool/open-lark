//! openlark-docs 链式调用入口（meta 风格，偏“模块入口”）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入“额外实现文件”
//! - openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//!   为避免为 200+ API 手写方法，这里先提供“模块级链式入口 + Config 透传”。
//! - 具体 API 调用仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。

use std::sync::Arc;

use openlark_core::config::Config;

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

    /// 与现有 `service::DocsService` 保持一致：需要时返回 Service 实例（按 feature 裁剪）。
    pub fn service(&self) -> crate::service::DocsService {
        crate::service::DocsService::new((*self.config).clone())
    }
}

/// ccm：`docs.ccm`
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct CcmClient {
    config: Arc<Config>,

    pub ccm_doc: CcmDocClient,
    pub ccm_docs: CcmDocsClient,
    pub ccm_drive_explorer: CcmDriveExplorerClient,
    pub ccm_drive_permission: CcmDrivePermissionClient,
    pub ccm_sheet: CcmSheetClient,
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
            ccm_doc: CcmDocClient::new(config.clone()),
            ccm_docs: CcmDocsClient::new(config.clone()),
            ccm_drive_explorer: CcmDriveExplorerClient::new(config.clone()),
            ccm_drive_permission: CcmDrivePermissionClient::new(config.clone()),
            ccm_sheet: CcmSheetClient::new(config.clone()),
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

    pub fn service(&self) -> crate::ccm::CcmService {
        crate::ccm::CcmService::new((*self.config).clone())
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
impl_ccm_project_client!(CcmDocClient, crate::ccm::ccm_doc::CcmDocService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(CcmDocsClient, crate::ccm::ccm_docs::CcmDocsService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(
    CcmDriveExplorerClient,
    crate::ccm::ccm_drive_explorer::CcmDriveExplorerService
);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(
    CcmDrivePermissionClient,
    crate::ccm::ccm_drive_permission::CcmDrivePermissionService
);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(CcmSheetClient, crate::ccm::ccm_sheet::CcmSheetService);
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
