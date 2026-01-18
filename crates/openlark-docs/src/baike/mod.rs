#![allow(clippy::module_inception)]

/// 知识库服务模块
///
/// 提供企业知识库、Wiki管理功能。包含baike和lingo两个项目。
use openlark_core::config::Config;

// 导出v1版本实现和模型
pub mod baike;
pub mod lingo;
pub use baike::*;
pub use lingo::*;

pub mod models;
pub mod v1;

// 重新导出服务和v1版本
pub use v1::BaikeV1Service;

/// Baike 知识库服务（兼容旧接口）
#[derive(Debug, Clone)]
pub struct BaikeService {
    /// 配置信息
    config: Config,
}

impl BaikeService {
    /// 创建新的知识库服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取baike项目v1服务
    pub fn v1(&self) -> BaikeV1Service<'_> {
        BaikeV1Service::new(&self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baike_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = BaikeService::new(config.clone());

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }
}
