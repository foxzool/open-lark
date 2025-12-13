/// 知识库服务模块
///
/// 提供企业知识库、Wiki管理功能。
use openlark_core::config::Config;

/// 知识库服务
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
