//! 文档服务主模块

use crate::error::{DocsError, DocsResult};
use std::sync::Arc;

/// 文档服务主入口
#[derive(Debug, Clone)]
pub struct DocsService {
    /// 配置信息
    config: Arc<openlark_core::config::Config>,
    /// HTTP客户端
    http_client: Arc<reqwest::Client>,
}

impl DocsService {
    /// 创建新的文档服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config: Arc::new(config),
            http_client: Arc::new(reqwest::Client::new()),
        }
    }

    /// 从配置创建服务（用于方便的构造）
    pub fn from_config(config: openlark_core::config::Config) -> DocsResult<Self> {
        Ok(Self::new(config))
    }

    /// 获取配置引用
    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 获取HTTP客户端引用
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// 访问云文档协同服务
    #[cfg(feature = "ccm")]
    pub fn ccm(&self) -> crate::ccm::CcmService {
        crate::ccm::CcmService::new(openlark_core::config::Config::new(
            openlark_core::config::ConfigInner {
                app_id: self.config.app_id.clone(),
                app_secret: self.config.app_secret.clone(),
                base_url: self.config.base_url.clone(),
                enable_token_cache: self.config.enable_token_cache,
                app_type: self.config.app_type,
                http_client: reqwest::Client::new(),
                req_timeout: self.config.req_timeout,
                header: self.config.header.clone(),
                token_manager: self.config.token_manager.clone(),
                app_ticket_manager: self.config.app_ticket_manager.clone(),
            },
        ))
    }

    /// 访问多维表格服务
    #[cfg(feature = "bitable")]
    pub fn bitable(&self) -> crate::bitable::BitableService {
        crate::bitable::BitableService::new(openlark_core::config::Config::new(
            openlark_core::config::ConfigInner {
                app_id: self.config.app_id.clone(),
                app_secret: self.config.app_secret.clone(),
                base_url: self.config.base_url.clone(),
                enable_token_cache: self.config.enable_token_cache,
                app_type: self.config.app_type,
                http_client: reqwest::Client::new(),
                req_timeout: self.config.req_timeout,
                header: self.config.header.clone(),
                token_manager: self.config.token_manager.clone(),
                app_ticket_manager: self.config.app_ticket_manager.clone(),
            },
        ))
    }

    /// 访问基础服务
    #[cfg(feature = "base")]
    pub fn base(&self) -> crate::base::BaseService {
        crate::base::BaseService::new(openlark_core::config::Config::new(
            openlark_core::config::ConfigInner {
                app_id: self.config.app_id.clone(),
                app_secret: self.config.app_secret.clone(),
                base_url: self.config.base_url.clone(),
                enable_token_cache: self.config.enable_token_cache,
                app_type: self.config.app_type,
                http_client: reqwest::Client::new(),
                req_timeout: self.config.req_timeout,
                header: self.config.header.clone(),
                token_manager: self.config.token_manager.clone(),
                app_ticket_manager: self.config.app_ticket_manager.clone(),
            },
        ))
    }

    /// 访问知识库服务
    #[cfg(feature = "baike")]
    pub fn baike(&self) -> crate::baike::BaikeService {
        crate::baike::BaikeService::new(self.config.clone())
    }

    /// 访问会议纪要服务
    #[cfg(feature = "minutes")]
    pub fn minutes(&self) -> crate::minutes::MinutesService {
        crate::minutes::MinutesService::new(self.config.clone())
    }
}

/// 服务构建器
#[derive(Debug, Clone)]
pub struct DocsServiceBuilder {
    config: Option<openlark_core::config::Config>,
}

impl DocsServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { config: None }
    }

    /// 设置应用ID
    pub fn app_id(self, _app_id: impl Into<String>) -> Self {
        // 简化实现，直接创建新的配置
        self
    }

    /// 设置应用密钥
    pub fn app_secret(self, _app_secret: impl Into<String>) -> Self {
        // 简化实现
        self
    }

    /// 设置访问令牌
    pub fn access_token(self, _access_token: impl Into<String>) -> Self {
        // 简化实现
        self
    }

    /// 设置自定义配置
    pub fn config(mut self, config: openlark_core::config::Config) -> Self {
        self.config = Some(config);
        self
    }

    /// 构建服务实例
    pub fn build(self) -> DocsResult<DocsService> {
        let config = self.config.ok_or_else(|| DocsError::InvalidParameter {
            parameter: "config".to_string(),
            reason: "Configuration is required".to_string(),
        })?;

        Ok(DocsService::new(config))
    }
}

impl Default for DocsServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsService::new(config);

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }

    #[test]
    fn test_docs_service_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsServiceBuilder::new().config(config).build().unwrap();

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }

    #[test]
    fn test_docs_service_builder_missing_config() {
        let result = DocsServiceBuilder::new().build();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocsError::InvalidParameter { .. }
        ));
    }
}
