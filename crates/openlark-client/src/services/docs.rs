//! 📄 文档服务访问层
//!
//! 作为 `openlark-docs` crate 的薄包装层：
//! - 不在 openlark-client 内重复实现具体云文档 API
//! - 直接复用 `openlark-docs` 的强类型 API 与目录组织
//!
//! KISS：避免“mock 返回值”造成误用；如需具体 API，请通过 `raw()` 获取底层服务入口。

use crate::Config;

/// 📄 文档服务
#[derive(Debug, Clone)]
pub struct DocsService {
    inner: openlark_docs::service::DocsService,
}

impl DocsService {
    /// 创建新的文档服务实例
    pub fn new(config: &Config) -> Self {
        let core_config = openlark_core::config::Config::builder()
            .app_id(config.app_id.clone())
            .app_secret(config.app_secret.clone())
            .base_url(config.base_url.clone())
            .build();

        Self {
            inner: openlark_docs::service::DocsService::new(core_config),
        }
    }

    /// 获取 `openlark-docs` 原生服务入口
    pub fn raw(&self) -> &openlark_docs::service::DocsService {
        &self.inner
    }

    /// 获取底层 `openlark-core` 配置引用（便于调试）
    pub fn core_config(&self) -> &openlark_core::config::Config {
        self.inner.config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_service_creation() {
        let config = Config::builder()
            .app_id("test")
            .app_secret("test")
            .build()
            .unwrap();

        let service = DocsService::new(&config);
        assert_eq!(service.core_config().app_id(), "test");
        assert_eq!(service.core_config().base_url(), config.base_url);
    }
}

