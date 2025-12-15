/// 知识库服务
///
/// 基础服务架构，具体功能在后续版本中实现。

use openlark_core::{config::Config, trait_system::Service};

// 版本化API
pub mod v1;
pub mod v2;

/// 知识库服务
///
/// 基础服务架构，具体功能在后续版本中实现。
pub struct WikiService {
    config: Config,
}

impl WikiService {
    /// 创建知识库服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V1版本API
    pub fn v1(&self) -> crate::ccm::wiki::v1::WikiV1Service {
        crate::ccm::wiki::v1::WikiV1Service::new(self.config.clone())
    }

    /// 获取V2版本API
    pub fn v2(&self) -> crate::ccm::wiki::v2::WikiService {
        crate::ccm::wiki::v2::WikiService::new(self.config.clone())
    }
}

impl Service for WikiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "WikiService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_wiki_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            ;
        let service = WikiService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            ;
        let service = WikiService::new(config);

        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
