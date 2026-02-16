//! # openlark-docs 链式调用入口（简化为仅配置获取）
//!
//! ## 设计理念
//!
//! openlark-docs 涵盖多个 bizTag/Project（ccm/base/bitable/baike/minutes 等），
//! 提供简洁的配置获取入口，Request 构建仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。
//!
//! ## 推荐入口
//!
//! **公开入口** (推荐用户使用):
//! - `DocsClient` - 文档服务的唯一公开入口
//! - 示例: `DocsClient::new(config).ccm.config().clone()` 用于获取配置
//!
//! ## 推荐调用方式
//!
//! ```rust,ignore
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
//! // ✅ 推荐：获取配置后构建 Request
//! // 访问云盘服务
//! let config = docs.ccm.config().clone();
//! // let file = UploadAllRequest::new(config, ...).execute().await?;
//!
//! // 访问多维表格
//! let config = docs.base.bitable.config().clone();
//! // let table = CreateTableRequest::new(config, ...).execute().await?;
//!
//! // 访问知识库
//! let config = docs.ccm.wiki.config().clone();
//! // let node = CreateNodeRequest::new(config, ...).execute().await?;
//! ```

use openlark_core::config::Config;
use std::sync::Arc;

/// Docs 链式入口：`docs.ccm.config()` / `docs.base.bitable.config()`（按 feature 裁剪）
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
            minutes: MinutesClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// ccm：`docs.ccm`（云文档协同）
#[cfg(feature = "ccm-core")]
#[derive(Debug, Clone)]
pub struct CcmClient {
    config: Arc<Config>,
}

#[cfg(feature = "ccm-core")]
impl CcmClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

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

    #[cfg(feature = "bitable")]
    pub fn bitable(&self) -> BitableClient {
        BitableClient::new(self.config.clone())
    }
}

/// bitable：`docs.base.bitable`（多维表格）
#[cfg(feature = "bitable")]
#[derive(Debug, Clone)]
pub struct BitableClient {
    config: Arc<Config>,
}

#[cfg(feature = "bitable")]
impl BitableClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
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
}

/// minutes：`docs.minutes`（会议纪要）
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
}
