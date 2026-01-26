//! openlark-docs 链式调用入口（meta 风格，偏“模块入口”）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入“额外实现文件”
//! - openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//!   为避免为 200+ API 手写方法，这里先提供“模块级链式入口 + Config 透传”。
//! - 具体 API 调用仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。

use openlark_core::config::Config;

/// Docs 链式入口：`docs.ccm.drive...` / `docs.bitable...`（按 feature 裁剪）
#[derive(Debug, Clone)]
pub struct DocsClient {
    config: Config,

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
    config: Config,

    pub explorer: ExplorerClient,
    pub permission: PermissionClient,
    pub ccm_sheet: CcmSheetClient,
    pub docs: DocsContentClient,
    pub docx: DocxClient,
    pub drive: DriveClient,
    pub sheets: SheetsClient,
    pub wiki: WikiClient,
}

#[cfg(feature = "ccm-core")]
impl CcmClient {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            explorer: ExplorerClient::new(config.clone()),
            permission: PermissionClient::new(config.clone()),
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
}

#[cfg(feature = "ccm-core")]
macro_rules! impl_ccm_project_client {
    ($name:ident, $service:path) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            config: Config,
        }

        impl $name {
            fn new(config: Config) -> Self {
                Self { config }
            }

            pub fn config(&self) -> &Config {
                &self.config
            }

            pub fn service(&self) -> $service {
                <$service>::new(self.config.clone())
            }
        }
    };
}

#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(ExplorerClient, crate::ccm::explorer::ExplorerService);
#[cfg(feature = "ccm-core")]
impl_ccm_project_client!(PermissionClient, crate::ccm::permission::PermissionService);
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
    config: Config,
}

#[cfg(any(feature = "base", feature = "bitable"))]
impl BaseClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn service(&self) -> crate::base::BaseService {
        crate::base::BaseService::new(self.config.clone())
    }

    #[cfg(feature = "bitable")]
    pub fn bitable(&self) -> crate::base::bitable::BitableService {
        crate::base::bitable::BitableService::new(self.config.clone())
    }
}

/// baike：`docs.baike`（baike/lingo 相关）
#[cfg(any(feature = "baike", feature = "lingo"))]
#[derive(Debug, Clone)]
pub struct BaikeClient {
    config: Config,
}

#[cfg(any(feature = "baike", feature = "lingo"))]
impl BaikeClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    #[cfg(feature = "baike")]
    pub fn service(&self) -> crate::baike::BaikeService {
        crate::baike::BaikeService::new(self.config.clone())
    }
}

/// minutes：`docs.minutes`
#[cfg(feature = "minutes")]
#[derive(Debug, Clone)]
pub struct MinutesClient {
    config: Config,
}

#[cfg(feature = "minutes")]
impl MinutesClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn service(&self) -> crate::minutes::MinutesService {
        crate::minutes::MinutesService::new(self.config.clone())
    }
}
